mod command;
mod config;
mod dsp_mode;
mod ffi;
mod input_stage;
mod parsers;
mod setup_info;
mod sweep;
mod tracking_status;

pub(crate) use command::Command;
pub use config::{CalcMode, Config, Mode, RadioModule};
pub use dsp_mode::DspMode;
pub use input_stage::InputStage;
pub use sweep::Sweep;
pub use tracking_status::TrackingStatus;

use crate::rf_explorer::{
    self, Callback, ConnectionResult, Device, Error, Frequency, Model, ParseFromBytes, Result,
    RfExplorer, SerialNumber, SerialPortReader, SetupInfo,
};
use num_enum::IntoPrimitive;
use serialport::SerialPortInfo;
use std::{
    fmt::Debug,
    io::{self, BufRead, ErrorKind},
    ops::RangeInclusive,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Instant,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive)]
#[repr(u8)]
pub enum WifiBand {
    TwoPointFourGhz = 1,
    FiveGhz,
}

pub struct SpectrumAnalyzer {
    serial_port: Arc<Mutex<SerialPortReader>>,
    is_reading: Arc<Mutex<bool>>,
    read_thread_handle: Option<JoinHandle<()>>,
    config: Arc<Mutex<Config>>,
    config_callback: Arc<Mutex<Callback<Config>>>,
    sweep: Arc<Mutex<Option<Sweep>>>,
    sweep_callback: Arc<Mutex<Callback<Sweep>>>,
    dsp_mode: Arc<Mutex<Option<DspMode>>>,
    serial_number: Arc<Mutex<Option<SerialNumber>>>,
    tracking_status: Arc<Mutex<Option<TrackingStatus>>>,
    input_stage: Arc<Mutex<Option<InputStage>>>,
    setup_info: SetupInfo,
    port_name: String,
}

impl SpectrumAnalyzer {
    const MIN_MAX_AMP_RANGE_DBM: RangeInclusive<i16> = -120..=35;
    const MIN_SWEEP_POINTS: u16 = 112;
    const EEOT_BYTES: [u8; 5] = [255, 254, 255, 254, 0];

    /// Spawns a new thread to read messages from the spectrum analyzer.
    fn start_read_thread(&mut self) {
        assert!(self.read_thread_handle.is_none());

        let is_reading = self.is_reading.clone();
        let serial_port = self.serial_port.clone();
        let sweep = self.sweep.clone();
        let sweep_callback = self.sweep_callback.clone();
        let config = self.config.clone();
        let config_callback = self.config_callback.clone();
        let dsp_mode = self.dsp_mode.clone();
        let serial_number = self.serial_number.clone();
        let tracking_status = self.tracking_status.clone();
        let input_stage = self.input_stage.clone();

        self.read_thread_handle = Some(thread::spawn(move || {
            let mut message_buf = Vec::new();
            *is_reading.lock().unwrap() = true;
            while *is_reading.lock().unwrap() {
                let read_message_result = serial_port
                    .lock()
                    .unwrap()
                    .read_until(b'\n', &mut message_buf);

                // Time out errors are recoverable so we should try to read again
                // Other errors are not recoverable and we should exit the thread
                if let Err(error) = read_message_result {
                    match error.kind() {
                        ErrorKind::TimedOut => continue,
                        _ => break,
                    }
                }

                // Try to parse a sweep from the message we received
                let parse_sweep_result = Sweep::parse_from_bytes(&message_buf);
                if let Ok((_, new_sweep)) = parse_sweep_result {
                    let mut sweep = sweep.lock().unwrap();
                    *sweep = Some(new_sweep);
                    if let Some(cb) = sweep_callback.lock().unwrap().as_mut() {
                        if let Some(sweep) = sweep.as_ref() {
                            cb(sweep.clone());
                        }
                    }
                    message_buf.clear();
                    continue;
                } else if let Err(nom::Err::Incomplete(_)) = parse_sweep_result {
                    // Check for Early-End-of-Transmission (EEOT) byte sequence
                    if let Some(eeot_index) = message_buf
                        .windows(5)
                        .position(|window| window == Self::EEOT_BYTES)
                    {
                        message_buf.drain(0..eeot_index + 5);
                    } else {
                        continue;
                    }
                }

                // Try to parse a config from the message we received
                if let Ok((_, new_config)) = Config::parse_from_bytes(&message_buf) {
                    let mut config = config.lock().unwrap();
                    *config = new_config;
                    if let Some(cb) = config_callback.lock().unwrap().as_mut() {
                        cb(*config);
                    }
                    message_buf.clear();
                    continue;
                }

                // Try to parse a DSP mode message from the message we received
                if let Ok((_, new_dsp_mode)) = DspMode::parse_from_bytes(&message_buf) {
                    *dsp_mode.lock().unwrap() = Some(new_dsp_mode);
                    message_buf.clear();
                    continue;
                }

                // Try to parse a serial number message from the message we received
                if let Ok((_, new_serial_number)) = SerialNumber::parse_from_bytes(&message_buf) {
                    serial_number.lock().unwrap().replace(new_serial_number);
                    message_buf.clear();
                    continue;
                }

                // Try to parse a tracking status message from the message we received
                if let Ok((_, new_tracking_status)) = TrackingStatus::parse_from_bytes(&message_buf)
                {
                    tracking_status.lock().unwrap().replace(new_tracking_status);
                    message_buf.clear();
                    continue;
                }

                // Try to parse an input stage message from the message we received
                if let Ok((_, new_input_stage)) = InputStage::parse_from_bytes(&message_buf) {
                    input_stage.lock().unwrap().replace(new_input_stage);
                    message_buf.clear();
                    continue;
                }

                // We weren't able to parse the message we received so clear the message buffer and read again
                message_buf.clear();
            }

            *is_reading.lock().unwrap() = false;
        }));
    }
}

impl Device for SpectrumAnalyzer {
    type Config = Config;
    type SetupInfo = SetupInfo<SpectrumAnalyzer>;

    fn connect(serial_port_info: &SerialPortInfo) -> ConnectionResult<Self> {
        let mut serial_port = rf_explorer::open(serial_port_info)?;

        let (config, setup_info) = SpectrumAnalyzer::read_setup_and_config(&mut serial_port)?;

        let mut spectrum_analyzer = SpectrumAnalyzer {
            serial_port: Arc::new(Mutex::new(serial_port)),
            is_reading: Arc::new(Mutex::new(false)),
            read_thread_handle: None,
            config: Arc::new(Mutex::new(config)),
            config_callback: Arc::new(Mutex::new(None)),
            sweep: Arc::new(Mutex::new(None)),
            sweep_callback: Arc::new(Mutex::new(None)),
            dsp_mode: Arc::new(Mutex::new(None)),
            serial_number: Arc::new(Mutex::new(None)),
            tracking_status: Arc::new(Mutex::new(None)),
            input_stage: Arc::new(Mutex::new(None)),
            setup_info,
            port_name: serial_port_info.port_name.clone(),
        };

        spectrum_analyzer.start_read_thread();

        // Wait until the read thread has started before returning
        while !*spectrum_analyzer.is_reading.lock().unwrap() {}

        Ok(spectrum_analyzer)
    }

    fn send_bytes(&mut self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
        self.serial_port
            .lock()
            .unwrap()
            .get_mut()
            .write_all(bytes.as_ref())
    }

    fn port_name(&self) -> &str {
        &self.port_name
    }

    fn setup_info(&self) -> &SetupInfo<Self> {
        &self.setup_info
    }

    fn serial_number(&self) -> Option<SerialNumber> {
        self.serial_number.lock().unwrap().clone()
    }
}

impl RfExplorer<SpectrumAnalyzer> {
    /// Returns a copy of the latest sweep received by the spectrum analyzer.
    pub fn latest_sweep(&self) -> Option<Sweep> {
        self.device.sweep.lock().unwrap().clone()
    }

    /// Returns a copy of the spectrum analyzer's current config.
    pub fn config(&self) -> Config {
        *self.device.config.lock().unwrap()
    }

    /// Returns the `Model` of the RF Explorer's main module.
    pub fn main_module_model(&self) -> Model {
        self.device.setup_info().main_module_model
    }

    /// Returns the `Model` of the RF Explorer's expansion module.
    pub fn expansion_module_model(&self) -> Model {
        self.device.setup_info().expansion_module_model
    }

    /// Returns the RF Explorer's firmware version.
    pub fn firmware_version(&self) -> &str {
        &self.device.setup_info().firmware_version
    }

    /// Returns the spectrum analyzer's DSP mode.
    pub fn dsp_mode(&self) -> Option<DspMode> {
        *self.device.dsp_mode.lock().unwrap()
    }

    /// Returns the status of tracking mode (enabled or disabled).
    pub fn tracking_status(&self) -> Option<TrackingStatus> {
        *self.device.tracking_status.lock().unwrap()
    }

    pub fn input_stage(&self) -> Option<InputStage> {
        *self.device.input_stage.lock().unwrap()
    }

    /// Returns which radio module is active (main or expansion)
    pub fn active_module(&self) -> RadioModule {
        self.config().active_radio_module
    }

    /// Returns which radio module is inactive (main or expansion)
    pub fn inactive_module(&self) -> Option<RadioModule> {
        if self.expansion_module_model() != Model::None {
            match self.config().active_radio_module {
                RadioModule::Main => Some(RadioModule::Expansion),
                RadioModule::Expansion => Some(RadioModule::Main),
            }
        } else {
            None
        }
    }

    /// Returns the model of the active RF Explorer radio module.
    pub fn active_module_model(&self) -> Model {
        match self.config().active_radio_module {
            RadioModule::Main => self.main_module_model(),
            RadioModule::Expansion => self.expansion_module_model(),
        }
    }

    /// Returns the model of the inactive RF Explorer radio module.
    pub fn inactive_module_model(&self) -> Model {
        match self.config().active_radio_module {
            RadioModule::Main => self.expansion_module_model(),
            RadioModule::Expansion => self.main_module_model(),
        }
    }

    /// Switches the spectrum analyzer's active module to the main module.
    pub fn use_main_module(&mut self) -> Result<()> {
        self.send_command(Command::SwitchModuleMain)?;

        let start_time = Instant::now();
        while start_time.elapsed() <= SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT {
            match self.config().active_radio_module {
                RadioModule::Main => return Ok(()),
                RadioModule::Expansion => continue,
            }
        }

        Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
    }

    /// Switches the spectrum analyzer's active module to the expansion module.
    pub fn use_expansion_module(&mut self) -> Result<()> {
        self.send_command(Command::SwitchModuleExp)?;

        let start_time = Instant::now();
        while start_time.elapsed() <= SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT {
            match self.config().active_radio_module {
                RadioModule::Expansion => return Ok(()),
                RadioModule::Main => continue,
            }
        }

        Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
    }

    /// Starts the spectrum analyzer's Wi-Fi analyzer.
    pub fn start_wifi_analyzer(&mut self, wifi_band: WifiBand) -> io::Result<()> {
        self.send_command(Command::StartWifiAnalyzer(wifi_band))
    }

    /// Stops the spectrum analyzer's Wi-Fi analyzer.
    pub fn stop_wifi_analyzer(&mut self) -> io::Result<()> {
        self.send_command(Command::StopWifiAnalyzer)
    }

    /// Requests the spectrum analyzer enter tracking mode.
    pub fn request_tracking(&mut self, start_hz: u64, step_hz: u64) -> Result<TrackingStatus> {
        // Set the tracking status to None so we can tell whether or not we've received a new
        // tracking status message by checking for Some
        *self.device.tracking_status.lock().unwrap() = None;

        // Send the command to enter tracking mode
        self.send_command(Command::StartTracking {
            start_freq: Frequency::from_hz(start_hz),
            step_freq: Frequency::from_hz(step_hz),
        })?;

        // Wait to see if we receive a DSP mode message in response
        let start_time = Instant::now();
        while start_time.elapsed() <= SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT {
            if let Some(&tracking_status) = self.device.tracking_status.lock().unwrap().as_ref() {
                return Ok(tracking_status);
            }
        }

        Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
    }

    /// Steps over the tracking step frequency and makes a measurement.
    pub fn tracking_step(&mut self, step: u16) -> io::Result<()> {
        self.send_command(Command::TrackingStep(step))
    }

    /// Sets the start and stop frequency of sweeps measured by the spectrum analyzer.
    pub fn set_start_stop(
        &mut self,
        start_freq: impl Into<Frequency>,
        stop_freq: impl Into<Frequency>,
    ) -> Result<Config> {
        let config = self.config();
        self.set_config(
            start_freq.into(),
            stop_freq.into(),
            config.min_amp_dbm,
            config.max_amp_dbm,
        )
    }

    /// Sets the center frequency and span of sweeps measured by the spectrum analyzer.
    pub fn set_center_span(
        &mut self,
        center_freq: impl Into<Frequency>,
        span_freq: impl Into<Frequency>,
    ) -> Result<Config> {
        let center_freq = center_freq.into();
        let span_freq = span_freq.into();
        self.set_start_stop(center_freq - span_freq / 2, center_freq + span_freq / 2)
    }

    /// Sets the minimum and maximum amplitudes displayed on the RF Explorer's screen.
    pub fn set_min_max_amps(&mut self, min_amp_dbm: i16, max_amp_dbm: i16) -> Result<Config> {
        let config = self.config();
        self.set_config(
            config.start_freq,
            config.stop_freq,
            min_amp_dbm,
            max_amp_dbm,
        )
    }

    /// Sets the spectrum analyzer's configuration.
    fn set_config(
        &mut self,
        start_freq: Frequency,
        stop_freq: Frequency,
        min_amp_dbm: i16,
        max_amp_dbm: i16,
    ) -> Result<Config> {
        self.validate_start_stop(start_freq, stop_freq)?;
        self.validate_min_max_amps(min_amp_dbm, max_amp_dbm)?;

        // Store a copy of the original config to help determine when we've received a new config
        let original_config = self.config();

        // Send the command to change the config
        self.send_command(Command::SetConfig {
            start_freq,
            stop_freq,
            min_amp_dbm,
            max_amp_dbm,
        })?;

        // Wait to see if we receive a new config in response
        let start_time = Instant::now();
        while start_time.elapsed() < SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT {
            let new_config = self.config();
            // If the new config is different than the old config it means we received a new config
            // in reponse to our command
            if new_config != original_config {
                return Ok(new_config);
            }
        }

        Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
    }

    /// Sets the callback that is called when the spectrum analyzer receives a `Sweep`.
    pub fn set_sweep_callback(&self, cb: impl FnMut(Sweep) + Send + 'static) {
        *self.device.sweep_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Sets the callback that is called when the spectrum analyzer receives a `Config`.
    pub fn set_config_callback(&self, cb: impl FnMut(Config) + Send + 'static) {
        *self.device.config_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Sets the number of points in each sweep measured by the spectrum analyzer.
    pub fn set_sweep_points(&mut self, sweep_points: u16) -> Result<Config> {
        // Only 'Plus' models can set the number of points in a sweep
        if !self.active_module_model().is_plus_model() {
            return Err(Error::InvalidOperation(
                "Only RF Explorer 'Plus' models support setting the number of sweep points"
                    .to_string(),
            ));
        }

        if sweep_points <= 4096 {
            self.send_command(Command::SetSweepPointsExt(sweep_points))?;
        } else {
            self.send_command(Command::SetSweepPointsLarge(sweep_points))?;
        }

        // The requested number of sweep points gets rounded down to a number that's a multiple of 16
        let expected_sweep_points = u32::from(if sweep_points < 112 {
            SpectrumAnalyzer::MIN_SWEEP_POINTS
        } else {
            (sweep_points / 16) * 16
        });

        // Wait until the current config shows the requested number of sweep points
        let start_time = Instant::now();
        while start_time.elapsed() < SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT {
            if self.config().sweep_points == expected_sweep_points {
                return Ok(self.config());
            }
        }

        Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
    }

    /// Sets the spectrum analyzer's calculator mode.
    pub fn set_calc_mode(&mut self, calc_mode: CalcMode) -> io::Result<()> {
        self.send_command(Command::SetCalcMode(calc_mode))
    }

    /// Sets the spectrum analyzer's input stage.
    pub fn set_input_stage(&mut self, input_stage: InputStage) -> io::Result<()> {
        self.send_command(Command::SetInputStage(input_stage))
    }

    /// Adds or subtracts an offset to the amplitudes in each sweep.
    pub fn set_offset_db(&mut self, offset_db: i8) -> io::Result<()> {
        self.send_command(Command::SetOffsetDB(offset_db))
    }

    /// Sets the spectrum analyzer's DSP mode.
    pub fn set_dsp_mode(&mut self, dsp_mode: DspMode) -> Result<()> {
        // Set the DSP mode to None so we can tell whether or not we've received a new DSP mode by
        // checking for Some
        *self.device.dsp_mode.lock().unwrap() = None;

        // Send the command to set the DSP mode
        self.send_command(Command::SetDsp(dsp_mode))?;

        // Wait to see if we receive a DSP mode message in response
        let start_time = Instant::now();
        while start_time.elapsed() <= SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT {
            if self.dsp_mode().is_some() {
                return Ok(());
            }
        }

        Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
    }

    fn validate_start_stop(&self, start_freq: Frequency, stop_freq: Frequency) -> Result<()> {
        if start_freq >= stop_freq {
            return Err(Error::InvalidInput(
                "The start frequency must be less than the stop frequency".to_string(),
            ));
        }

        let active_model = self.active_module_model();

        let min_max_freq = active_model.min_freq()..=active_model.max_freq();
        if !min_max_freq.contains(&start_freq) {
            return Err(Error::InvalidInput(format!(
                "The start frequency {} MHz is not within the RF Explorer's frequency range of {}-{} MHz",
                start_freq.as_mhz_f64(),
                min_max_freq.start().as_mhz_f64(),
                min_max_freq.end().as_mhz_f64()
            )));
        } else if !min_max_freq.contains(&stop_freq) {
            return Err(Error::InvalidInput(format!(
                "The stop frequency {} MHz is not within the RF Explorer's frequency range of {}-{} MHz",
                stop_freq.as_mhz(),
                min_max_freq.start().as_mhz_f64(),
                min_max_freq.end().as_mhz_f64()
            )));
        }

        let min_max_span = active_model.min_span()..=active_model.max_span();
        if !min_max_span.contains(&(stop_freq - start_freq)) {
            return Err(Error::InvalidInput(format!(
                "The span {} MHz is not within the RF Explorer's span range of {}-{} MHz",
                (stop_freq - start_freq).as_mhz_f64(),
                min_max_span.start().as_mhz_f64(),
                min_max_span.end().as_mhz_f64()
            )));
        }

        Ok(())
    }

    fn validate_min_max_amps(&self, min_amp_dbm: i16, max_amp_dbm: i16) -> Result<()> {
        // The bottom amplitude must be less than the top amplitude
        if min_amp_dbm >= max_amp_dbm {
            return Err(Error::InvalidInput(
                "The minimum amplitude must be less than the maximum amplitude".to_string(),
            ));
        }

        // The top and bottom amplitude must be within the RF Explorer's min and max amplitude range
        if !SpectrumAnalyzer::MIN_MAX_AMP_RANGE_DBM.contains(&min_amp_dbm) {
            return Err(Error::InvalidInput(format!(
                "The amplitude {} dBm is not within the RF Explorer's amplitude range of {}-{} dBm",
                min_amp_dbm,
                SpectrumAnalyzer::MIN_MAX_AMP_RANGE_DBM.start(),
                SpectrumAnalyzer::MIN_MAX_AMP_RANGE_DBM.end()
            )));
        } else if !SpectrumAnalyzer::MIN_MAX_AMP_RANGE_DBM.contains(&max_amp_dbm) {
            return Err(Error::InvalidInput(format!(
                "The amplitude {} dBm is not within the RF Explorer's amplitude range of {}-{} dBm",
                max_amp_dbm,
                SpectrumAnalyzer::MIN_MAX_AMP_RANGE_DBM.start(),
                SpectrumAnalyzer::MIN_MAX_AMP_RANGE_DBM.end()
            )));
        }

        Ok(())
    }
}

impl Drop for SpectrumAnalyzer {
    fn drop(&mut self) {
        *self.is_reading.lock().unwrap() = false;
        if let Some(read_handle) = self.read_thread_handle.take() {
            let _ = read_handle.join();
        }
    }
}

impl Debug for SpectrumAnalyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectrumAnalyzer")
            .field("setup_info", &self.setup_info)
            .field("config", &self.config.lock().unwrap())
            .finish()
    }
}
