mod command;
mod config;
mod dsp_mode;
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
    self, Callback, ConnectionError, ConnectionResult, Device, Error, Frequency, Model,
    ParseFromBytes, Result, RfExplorer, SerialNumber, SerialPortReader, SetupInfo,
};
use num_enum::IntoPrimitive;
use serialport::SerialPortInfo;
use std::{
    fmt::Debug,
    io::{self, BufRead, ErrorKind},
    ops::RangeInclusive,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive)]
#[repr(u8)]
pub enum WifiBand {
    TwoPointFourGhz = 1,
    FiveGhz,
}

pub struct SpectrumAnalyzer {
    serial_port: Arc<Mutex<SerialPortReader>>,
    is_reading_condvar_pair: Arc<(Mutex<bool>, Condvar)>,
    read_thread_handle: Option<JoinHandle<()>>,
    config_condvar_pair: Arc<(Mutex<Config>, Condvar)>,
    config_callback: Arc<Mutex<Callback<Config>>>,
    sweep_condvar_pair: Arc<(Mutex<Option<Sweep>>, Condvar)>,
    sweep_callback: Arc<Mutex<Callback<Sweep>>>,
    dsp_mode_condvar_pair: Arc<(Mutex<Option<DspMode>>, Condvar)>,
    tracking_status_condvar_pair: Arc<(Mutex<Option<TrackingStatus>>, Condvar)>,
    input_stage: Arc<Mutex<Option<InputStage>>>,
    setup_info: SetupInfo,
    serial_number: SerialNumber,
    port_name: String,
}

impl SpectrumAnalyzer {
    const MIN_MAX_AMP_RANGE_DBM: RangeInclusive<i16> = -120..=35;
    const MIN_SWEEP_POINTS: u16 = 112;
    const EEOT_BYTES: [u8; 5] = [255, 254, 255, 254, 0];
    const NEXT_SWEEP_TIMEOUT: Duration = Duration::from_secs(2);
    const START_READ_THREAD_TIMEOUT: Duration = Duration::from_secs(1);

    /// Spawns a new thread to read messages from the spectrum analyzer.
    fn start_read_thread(&mut self) {
        assert!(self.read_thread_handle.is_none());

        let is_reading_condvar_pair = self.is_reading_condvar_pair.clone();
        let serial_port = self.serial_port.clone();
        let sweep_condvar_pair = self.sweep_condvar_pair.clone();
        let sweep_callback = self.sweep_callback.clone();
        let config_condvar_pair = self.config_condvar_pair.clone();
        let config_callback = self.config_callback.clone();
        let dsp_mode_condvar_pair = self.dsp_mode_condvar_pair.clone();
        let tracking_status_condvar_pair = self.tracking_status_condvar_pair.clone();
        let input_stage = self.input_stage.clone();

        self.read_thread_handle = Some(thread::spawn(move || {
            let mut message_buf = Vec::new();
            *is_reading_condvar_pair.0.lock().unwrap() = true;
            is_reading_condvar_pair.1.notify_all();
            while *is_reading_condvar_pair.0.lock().unwrap() {
                let read_message_result = serial_port
                    .lock()
                    .unwrap()
                    .read_until(b'\n', &mut message_buf);

                // Time out errors are recoverable so we should try to read again
                // Other errors are not recoverable and we should exit the thread
                match read_message_result {
                    Ok(bytes_read) => trace!("Read {} bytes", bytes_read),
                    Err(e) if e.kind() == ErrorKind::TimedOut => {
                        continue;
                    }
                    Err(e) => {
                        break;
                    }
                };

                // Try to parse a sweep from the message we received
                let parse_sweep_result = Sweep::parse_from_bytes(&message_buf);
                if let Ok((_, sweep)) = parse_sweep_result {
                    *sweep_condvar_pair.0.lock().unwrap() = Some(sweep);
                    if let Some(cb) = sweep_callback.lock().unwrap().as_mut() {
                        if let Some(sweep) = sweep_condvar_pair.0.lock().unwrap().as_ref() {
                            cb(sweep.clone());
                        }
                    }
                    sweep_condvar_pair.1.notify_one();
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
                if let Ok((_, config)) = Config::parse_from_bytes(&message_buf) {
                    *config_condvar_pair.0.lock().unwrap() = config;
                    if let Some(cb) = config_callback.lock().unwrap().as_mut() {
                        cb(*config_condvar_pair.0.lock().unwrap());
                    }
                    config_condvar_pair.1.notify_one();
                    message_buf.clear();
                    continue;
                }

                // Try to parse a DSP mode message from the message we received
                if let Ok((_, dsp_mode)) = DspMode::parse_from_bytes(&message_buf) {
                    *dsp_mode_condvar_pair.0.lock().unwrap() = Some(dsp_mode);
                    dsp_mode_condvar_pair.1.notify_one();
                    message_buf.clear();
                    continue;
                }

                // Try to parse a tracking status message from the message we received
                if let Ok((_, tracking_status)) = TrackingStatus::parse_from_bytes(&message_buf) {
                    *tracking_status_condvar_pair.0.lock().unwrap() = Some(tracking_status);
                    tracking_status_condvar_pair.1.notify_one();
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

            *is_reading_condvar_pair.0.lock().unwrap() = false;
            is_reading_condvar_pair.1.notify_all();
        }));
    }
}

impl Device for SpectrumAnalyzer {
    type Config = Config;
    type SetupInfo = SetupInfo<SpectrumAnalyzer>;

    fn connect(serial_port_info: &SerialPortInfo) -> ConnectionResult<Self> {
        let mut serial_port = rf_explorer::open(serial_port_info)?;

        let (config, setup_info, serial_number) =
            SpectrumAnalyzer::read_initial_messages(&mut serial_port)?;

        let mut spectrum_analyzer = SpectrumAnalyzer {
            serial_port: Arc::new(Mutex::new(serial_port)),
            is_reading_condvar_pair: Arc::new((Mutex::new(false), Condvar::new())),
            read_thread_handle: None,
            config_condvar_pair: Arc::new((Mutex::new(config), Condvar::new())),
            config_callback: Arc::new(Mutex::new(None)),
            sweep_condvar_pair: Arc::new((Mutex::new(None), Condvar::new())),
            sweep_callback: Arc::new(Mutex::new(None)),
            dsp_mode_condvar_pair: Arc::new((Mutex::new(None), Condvar::new())),
            tracking_status_condvar_pair: Arc::new((Mutex::new(None), Condvar::new())),
            input_stage: Arc::new(Mutex::new(None)),
            setup_info,
            serial_number,
            port_name: serial_port_info.port_name.clone(),
        };

        spectrum_analyzer.start_read_thread();

        // Check if the read thread has already started
        let is_reading = spectrum_analyzer.is_reading_condvar_pair.0.lock().unwrap();
        if *is_reading {
            drop(is_reading);
            return Ok(spectrum_analyzer);
        }

        // Wait until the read thread has started before returning
        let condvar = &spectrum_analyzer.is_reading_condvar_pair.1;
        let (is_reading, timeout_result) = condvar
            .wait_timeout_while(
                is_reading,
                SpectrumAnalyzer::START_READ_THREAD_TIMEOUT,
                |is_reading| !*is_reading,
            )
            .unwrap();
        drop(is_reading);

        if !timeout_result.timed_out() {
            Ok(spectrum_analyzer)
        } else {
            Err(ConnectionError::Io(io::ErrorKind::TimedOut.into()))
        }
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

    fn serial_number(&self) -> SerialNumber {
        self.serial_number.clone()
    }
}

impl RfExplorer<SpectrumAnalyzer> {
    /// Returns a copy of the latest sweep received by the spectrum analyzer.
    pub fn latest_sweep(&self) -> Option<Sweep> {
        self.device.sweep_condvar_pair.0.lock().unwrap().clone()
    }

    /// Waits for the RF Explorer to measure its next `Sweep`.
    pub fn wait_for_next_sweep(&self) -> Result<Sweep> {
        self.wait_for_next_sweep_with_timeout(SpectrumAnalyzer::NEXT_SWEEP_TIMEOUT)
    }

    /// Waits for the RF Explorer to measure its next `Sweep` or for the timeout duration to elapse.
    pub fn wait_for_next_sweep_with_timeout(&self, timeout: Duration) -> Result<Sweep> {
        let previous_sweep = self.latest_sweep();

        let (sweep, cond_var) = &*self.device.sweep_condvar_pair;
        let (sweep, timeout_result) = cond_var
            .wait_timeout_while(sweep.lock().unwrap(), timeout, |sweep| {
                *sweep == previous_sweep || sweep.is_none()
            })
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(sweep.clone().unwrap())
        } else {
            Err(Error::TimedOut(timeout))
        }
    }

    /// Returns a copy of the spectrum analyzer's current config.
    pub fn config(&self) -> Config {
        *self.device.config_condvar_pair.0.lock().unwrap()
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
        *self.device.dsp_mode_condvar_pair.0.lock().unwrap()
    }

    /// Returns the status of tracking mode (enabled or disabled).
    pub fn tracking_status(&self) -> Option<TrackingStatus> {
        *self.device.tracking_status_condvar_pair.0.lock().unwrap()
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

        // Check if the RF Explorer is already using the main module
        let config = self.device.config_condvar_pair.0.lock().unwrap();
        if config.active_radio_module == RadioModule::Main {
            return Ok(());
        }

        // Wait until the config shows that the main module is active
        let condvar = &self.device.config_condvar_pair.1;
        let (_, timeout_result) = condvar
            .wait_timeout_while(
                config,
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |config| config.active_radio_module != RadioModule::Main,
            )
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(())
        } else {
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
    }

    /// Switches the spectrum analyzer's active module to the expansion module.
    pub fn use_expansion_module(&mut self) -> Result<()> {
        self.send_command(Command::SwitchModuleExp)?;

        // Check if the RF Explorer is already using the main module
        let config = self.device.config_condvar_pair.0.lock().unwrap();
        if config.active_radio_module == RadioModule::Expansion {
            return Ok(());
        }

        // Wait until the config shows that the expansion module is active
        let condvar = &self.device.config_condvar_pair.1;
        let (_, timeout_result) = condvar
            .wait_timeout_while(
                config,
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |config| config.active_radio_module != RadioModule::Expansion,
            )
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(())
        } else {
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
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
        *self.device.tracking_status_condvar_pair.0.lock().unwrap() = None;

        // Send the command to enter tracking mode
        self.send_command(Command::StartTracking {
            start_freq: Frequency::from_hz(start_hz),
            step_freq: Frequency::from_hz(step_hz),
        })?;

        // Wait to see if we receive a tracking status message in response
        let condvar = &self.device.tracking_status_condvar_pair.1;
        let (tracking_status, timeout_result) = condvar
            .wait_timeout_while(
                self.device.tracking_status_condvar_pair.0.lock().unwrap(),
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |tracking_status| tracking_status.is_some(),
            )
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(tracking_status.unwrap())
        } else {
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
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
    ) -> Result<()> {
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
    ) -> Result<()> {
        let center_freq = center_freq.into();
        let span_freq = span_freq.into();
        self.set_start_stop(center_freq - span_freq / 2, center_freq + span_freq / 2)
    }

    /// Sets the minimum and maximum amplitudes displayed on the RF Explorer's screen.
    pub fn set_min_max_amps(&mut self, min_amp_dbm: i16, max_amp_dbm: i16) -> Result<()> {
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
    ) -> Result<()> {
        self.validate_start_stop(start_freq, stop_freq)?;
        self.validate_min_max_amps(min_amp_dbm, max_amp_dbm)?;

        // Send the command to change the config
        self.send_command(Command::SetConfig {
            start_freq,
            stop_freq,
            min_amp_dbm,
            max_amp_dbm,
        })?;

        // Function to check whether a config contains the requested values
        let config_contains_requested_values = |config: &Config| {
            config.start_freq.abs_diff(start_freq) < config.step_freq
                && config.stop_freq.abs_diff(stop_freq) < config.step_freq
                && config.min_amp_dbm == min_amp_dbm
                && config.max_amp_dbm == max_amp_dbm
        };

        // Check if the current config already contains the requested values
        let config = self.device.config_condvar_pair.0.lock().unwrap();
        if config_contains_requested_values(&config) {
            return Ok(());
        }

        // Wait until the current config contains the requested values
        let condvar = &self.device.config_condvar_pair.1;
        let (_, timeout_result) = condvar
            .wait_timeout_while(
                config,
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |config| !config_contains_requested_values(config),
            )
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(())
        } else {
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
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
    pub fn set_sweep_points(&mut self, sweep_points: u16) -> Result<()> {
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

        // Check if the current config already contains the requested sweep points
        let (config, cond_var) = &*self.device.config_condvar_pair;
        if config.lock().unwrap().sweep_points == expected_sweep_points {
            return Ok(());
        }

        // Wait until the current config contains the requested sweep points
        let (_, timeout_result) = cond_var
            .wait_timeout_while(
                config.lock().unwrap(),
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |config| config.sweep_points != expected_sweep_points,
            )
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(())
        } else {
            warn!("Failed to receive updated config");
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
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
        *self.device.dsp_mode_condvar_pair.0.lock().unwrap() = None;

        // Send the command to set the DSP mode
        self.send_command(Command::SetDsp(dsp_mode))?;

        // Wait to see if we receive a DSP mode message in response
        let condvar = &self.device.dsp_mode_condvar_pair.1;
        let (_, timeout_result) = condvar
            .wait_timeout_while(
                self.device.dsp_mode_condvar_pair.0.lock().unwrap(),
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |dsp_mode| dsp_mode.is_some(),
            )
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(())
        } else {
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
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
        *self.is_reading_condvar_pair.0.lock().unwrap() = false;
        if let Some(read_handle) = self.read_thread_handle.take() {
            let _ = read_handle.join();
        }
    }
}

impl Debug for SpectrumAnalyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectrumAnalyzer")
            .field("port_name", &self.port_name)
            .field("setup_info", &self.setup_info)
            .field("config", &self.config_condvar_pair.0.lock().unwrap())
            .field("serial_number", &self.serial_number)
            .finish()
    }
}
