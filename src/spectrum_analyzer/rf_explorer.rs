use super::{
    CalcMode, Command, Config, DspMode, InputStage, Message, RadioModule, Sweep, TrackingStatus,
};
use crate::common::{
    self, Callback, ConnectionError, ConnectionResult, Device, Error, Frequency, Model, Result,
    RfExplorer, SerialNumber, SerialPortReader, SetupInfo,
};
use num_enum::IntoPrimitive;
use serialport::SerialPortInfo;
use std::{
    fmt::Debug,
    io::{self, BufRead},
    ops::RangeInclusive,
    sync::{Arc, Condvar, Mutex},
    thread::JoinHandle,
    time::Duration,
};
use tracing::{debug, error, info, warn};

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive)]
#[repr(u8)]
pub enum WifiBand {
    TwoPointFourGhz = 1,
    FiveGhz,
}

pub struct SpectrumAnalyzer {
    serial_port: Arc<Mutex<SerialPortReader>>,
    is_reading: Arc<Mutex<bool>>,
    read_thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    config: Arc<(Mutex<Option<Config>>, Condvar)>,
    config_callback: Arc<Mutex<Callback<Config>>>,
    sweep: Arc<(Mutex<Option<Sweep>>, Condvar)>,
    sweep_callback: Arc<Mutex<Callback<Sweep>>>,
    dsp_mode: Arc<(Mutex<Option<DspMode>>, Condvar)>,
    tracking_status: Arc<(Mutex<Option<TrackingStatus>>, Condvar)>,
    input_stage: Arc<(Mutex<Option<InputStage>>, Condvar)>,
    setup_info: Arc<(Mutex<Option<SetupInfo>>, Condvar)>,
    serial_number: Arc<(Mutex<Option<SerialNumber>>, Condvar)>,
    port_name: String,
}

impl Device for SpectrumAnalyzer {
    type Message = crate::spectrum_analyzer::Message;

    #[tracing::instrument]
    fn connect(serial_port_info: &SerialPortInfo) -> ConnectionResult<Arc<Self>> {
        let serial_port = common::open(serial_port_info)?;

        let device = Arc::new(SpectrumAnalyzer {
            serial_port: Arc::new(Mutex::new(serial_port)),
            is_reading: Arc::new(Mutex::new(true)),
            read_thread_handle: Arc::new(Mutex::new(None)),
            config: Arc::new((Mutex::new(None), Condvar::new())),
            config_callback: Arc::new(Mutex::new(None)),
            sweep: Arc::new((Mutex::new(None), Condvar::new())),
            sweep_callback: Arc::new(Mutex::new(None)),
            dsp_mode: Arc::new((Mutex::new(None), Condvar::new())),
            tracking_status: Arc::new((Mutex::new(None), Condvar::new())),
            input_stage: Arc::new((Mutex::new(None), Condvar::new())),
            setup_info: Arc::new((Mutex::new(None), Condvar::new())),
            serial_number: Arc::new((Mutex::new(None), Condvar::new())),
            port_name: serial_port_info.port_name.clone(),
        });

        *device.read_thread_handle.lock().unwrap() =
            Some(SpectrumAnalyzer::spawn_read_thread(device.clone()));

        // Wait to receive a Config before considering this a valid RF Explorer spectrum analyzer
        let (lock, cvar) = &*device.config;
        let (_, timeout_result) = cvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::RECEIVE_FIRST_CONFIG_TIMEOUT,
                |config| config.is_none(),
            )
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(device)
        } else {
            Err(ConnectionError::Io(io::ErrorKind::TimedOut.into()))
        }
    }

    fn read_line(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.serial_port.lock().unwrap().read_until(b'\n', buf)
    }

    fn is_reading(&self) -> bool {
        *self.is_reading.lock().unwrap()
    }

    fn process_message(&self, message: Message) {
        match message {
            Message::Config(config) => {
                *self.config.0.lock().unwrap() = Some(config);
                self.config.1.notify_one();
                if let Some(ref mut cb) = *self.config_callback.lock().unwrap() {
                    cb(config);
                }
            }
            Message::Sweep(sweep) => {
                *self.sweep.0.lock().unwrap() = Some(sweep);
                self.sweep.1.notify_one();
                if let Some(ref mut cb) = *self.sweep_callback.lock().unwrap() {
                    if let Some(ref sweep) = *self.sweep.0.lock().unwrap() {
                        cb(sweep.clone());
                    }
                }
            }
            Message::DspMode(dsp_mode) => {
                *self.dsp_mode.0.lock().unwrap() = Some(dsp_mode);
                self.dsp_mode.1.notify_one();
            }
            Message::InputStage(input_stage) => {
                *self.input_stage.0.lock().unwrap() = Some(input_stage);
                self.input_stage.1.notify_one();
            }
            Message::TrackingStatus(tracking_status) => {
                *self.tracking_status.0.lock().unwrap() = Some(tracking_status);
                self.tracking_status.1.notify_one();
            }
            Message::SerialNumber(serial_number) => {
                *self.serial_number.0.lock().unwrap() = Some(serial_number);
                self.serial_number.1.notify_one();
            }
            Message::SetupInfo(setup_info) => {
                *self.setup_info.0.lock().unwrap() = Some(setup_info);
                self.setup_info.1.notify_one();
            }
            _ => (),
        }
    }

    #[tracing::instrument(skip(self, bytes))]
    fn send_bytes(&self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
        debug!("Sending bytes: {:?}", bytes.as_ref());
        self.serial_port
            .lock()
            .unwrap()
            .get_mut()
            .write_all(bytes.as_ref())
    }

    fn port_name(&self) -> &str {
        &self.port_name
    }

    fn setup_info(&self) -> SetupInfo<Self> {
        self.setup_info
            .0
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_default()
    }

    fn serial_number(&self) -> SerialNumber {
        self.serial_number
            .0
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_default()
    }
}

impl Drop for SpectrumAnalyzer {
    fn drop(&mut self) {
        *self.is_reading.lock().unwrap() = false;
        if let Some(read_handle) = self.read_thread_handle.lock().unwrap().take() {
            let _ = read_handle.join();
        }
    }
}

impl Debug for SpectrumAnalyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectrumAnalyzer")
            .field("port_name", &self.port_name)
            .field("setup_info", &self.setup_info)
            .field("config", &self.config)
            .field("serial_number", &self.serial_number)
            .finish()
    }
}

impl RfExplorer<SpectrumAnalyzer> {
    const MIN_MAX_AMP_RANGE_DBM: RangeInclusive<i16> = -120..=35;
    const MIN_SWEEP_POINTS: u16 = 112;
    const NEXT_SWEEP_TIMEOUT: Duration = Duration::from_secs(2);

    /// Returns a copy of the latest sweep measured by the RF Explorer.
    pub fn latest_sweep(&self) -> Option<Sweep> {
        self.device.sweep.0.lock().unwrap().clone()
    }

    /// Waits for the RF Explorer to measure its next `Sweep`.
    pub fn wait_for_next_sweep(&self) -> Result<Sweep> {
        self.wait_for_next_sweep_with_timeout(Self::NEXT_SWEEP_TIMEOUT)
    }

    /// Waits for the RF Explorer to measure its next `Sweep` or for the timeout duration to elapse.
    pub fn wait_for_next_sweep_with_timeout(&self, timeout: Duration) -> Result<Sweep> {
        let previous_sweep = self.latest_sweep();

        let (sweep, cond_var) = &*self.device.sweep;
        let (_, timeout_result) = cond_var
            .wait_timeout_while(sweep.lock().unwrap(), timeout, |sweep| {
                *sweep == previous_sweep || sweep.is_none()
            })
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(self.latest_sweep().unwrap())
        } else {
            Err(Error::TimedOut(timeout))
        }
    }

    /// Returns a copy of the spectrum analyzer's current config.
    pub fn config(&self) -> Config {
        self.device.config.0.lock().unwrap().unwrap_or_default()
    }

    /// Returns the spectrum analyzer's DSP mode.
    pub fn dsp_mode(&self) -> Option<DspMode> {
        *self.device.dsp_mode.0.lock().unwrap()
    }

    /// Returns the status of tracking mode (enabled or disabled).
    pub fn tracking_status(&self) -> Option<TrackingStatus> {
        *self.device.tracking_status.0.lock().unwrap()
    }

    pub fn input_stage(&self) -> Option<InputStage> {
        *self.device.input_stage.0.lock().unwrap()
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
    #[tracing::instrument]
    pub fn use_main_module(&mut self) -> Result<()> {
        self.send_command(Command::SwitchModuleMain)?;

        // Check if the RF Explorer is already using the main module
        if self.config().active_radio_module == RadioModule::Main {
            return Ok(());
        }

        // Wait until the config shows that the main module is active
        let (lock, condvar) = &*self.device.config;
        let (_, timeout_result) = condvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |config| {
                    config
                        .filter(|config| config.active_radio_module == RadioModule::Main)
                        .is_none()
                },
            )
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(())
        } else {
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
    }

    /// Switches the spectrum analyzer's active module to the expansion module.
    #[tracing::instrument]
    pub fn use_expansion_module(&mut self) -> Result<()> {
        self.send_command(Command::SwitchModuleExp)?;

        // Check if the RF Explorer is already using the main module
        if self.config().active_radio_module == RadioModule::Expansion {
            return Ok(());
        }

        // Wait until the config shows that the expansion module is active
        let (lock, condvar) = &*self.device.config;
        let (_, timeout_result) = condvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |config| {
                    config
                        .filter(|config| config.active_radio_module == RadioModule::Expansion)
                        .is_none()
                },
            )
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(())
        } else {
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
    }

    /// Starts the spectrum analyzer's Wi-Fi analyzer.
    #[tracing::instrument]
    pub fn start_wifi_analyzer(&self, wifi_band: WifiBand) -> io::Result<()> {
        self.send_command(Command::StartWifiAnalyzer(wifi_band))
    }

    /// Stops the spectrum analyzer's Wi-Fi analyzer.
    #[tracing::instrument(skip(self))]
    pub fn stop_wifi_analyzer(&mut self) -> io::Result<()> {
        self.send_command(Command::StopWifiAnalyzer)
    }

    /// Requests the spectrum analyzer enter tracking mode.
    #[tracing::instrument(skip(self))]
    pub fn request_tracking(&self, start_hz: u64, step_hz: u64) -> Result<TrackingStatus> {
        // Set the tracking status to None so we can tell whether or not we've received a new
        // tracking status message by checking for Some
        *self.device.tracking_status.0.lock().unwrap() = None;

        // Send the command to enter tracking mode
        self.send_command(Command::StartTracking {
            start_freq: Frequency::from_hz(start_hz),
            step_freq: Frequency::from_hz(step_hz),
        })?;

        // Wait to see if we receive a tracking status message in response
        let (lock, condvar) = &*self.device.tracking_status;
        let (tracking_status, timeout_result) = condvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |tracking_status| tracking_status.is_some(),
            )
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(tracking_status.unwrap_or_default())
        } else {
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
    }

    /// Steps over the tracking step frequency and makes a measurement.
    #[tracing::instrument(skip(self))]
    pub fn tracking_step(&self, step: u16) -> io::Result<()> {
        self.send_command(Command::TrackingStep(step))
    }

    /// Sets the start and stop frequency of sweeps measured by the spectrum analyzer.
    pub fn set_start_stop(
        &self,
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

    /// Sets the start frequency, stop frequency, and number of points of sweeps measured by the spectrum analyzer.
    pub fn set_start_stop_sweep_points(
        &self,
        start: impl Into<Frequency>,
        stop: impl Into<Frequency>,
        sweep_points: u16,
    ) -> Result<()> {
        let (start, stop) = (start.into(), stop.into());
        let config = self.config();
        self.set_sweep_points(sweep_points)?;
        self.set_config(start, stop, config.min_amp_dbm, config.max_amp_dbm)
    }

    /// Sets the center frequency and span of sweeps measured by the spectrum analyzer.
    pub fn set_center_span(
        &self,
        center_freq: impl Into<Frequency>,
        span_freq: impl Into<Frequency>,
    ) -> Result<()> {
        let center_freq = center_freq.into();
        let span_freq = span_freq.into();
        self.set_start_stop(center_freq - span_freq / 2, center_freq + span_freq / 2)
    }

    /// Sets the center frequency, span, and number of points of sweeps measured by the spectrum analyzer.
    pub fn set_center_span_sweep_points(
        &self,
        center: impl Into<Frequency>,
        span: impl Into<Frequency>,
        sweep_points: u16,
    ) -> Result<()> {
        let (center, span) = (center.into(), span.into());
        self.set_start_stop_sweep_points(center - span / 2, center + span / 2, sweep_points)
    }

    /// Sets the minimum and maximum amplitudes displayed on the RF Explorer's screen.
    #[tracing::instrument(skip(self))]
    pub fn set_min_max_amps(&self, min_amp_dbm: i16, max_amp_dbm: i16) -> Result<()> {
        let config = self.config();
        self.set_config(
            config.start_freq,
            config.stop_freq,
            min_amp_dbm,
            max_amp_dbm,
        )
    }

    /// Sets the spectrum analyzer's configuration.
    #[tracing::instrument(skip(self))]
    fn set_config(
        &self,
        start_freq: Frequency,
        stop_freq: Frequency,
        min_amp_dbm: i16,
        max_amp_dbm: i16,
    ) -> Result<()> {
        info!("Validating start and stop frequencies");
        self.validate_start_stop(start_freq, stop_freq)?;
        info!("Validating min and max amplitudes");
        self.validate_min_max_amps(min_amp_dbm, max_amp_dbm)?;

        // Send the command to change the config
        info!("Sending 'SetConfig' command");
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
        if config_contains_requested_values(&self.config()) {
            return Ok(());
        }

        // Wait until the current config contains the requested values
        info!("Waiting to receive updated config");
        let (lock, condvar) = &*self.device.config;
        let (_, timeout_result) = condvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |config| config.filter(config_contains_requested_values).is_none(),
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
    #[tracing::instrument]
    pub fn set_sweep_points(&self, sweep_points: u16) -> Result<()> {
        // Only 'Plus' models can set the number of points in a sweep
        if !self.active_module_model().is_plus_model() {
            return Err(Error::InvalidOperation(
                "Only RF Explorer 'Plus' models support setting the number of sweep points"
                    .to_string(),
            ));
        }

        info!("Sending 'SetSweepPoints' command");
        if sweep_points <= 4096 {
            self.send_command(Command::SetSweepPointsExt(sweep_points))?;
        } else {
            self.send_command(Command::SetSweepPointsLarge(sweep_points))?;
        }

        // The requested number of sweep points gets rounded down to a number that's a multiple of 16
        let expected_sweep_points = if sweep_points < 112 {
            Self::MIN_SWEEP_POINTS
        } else {
            (sweep_points / 16) * 16
        };

        // Check if the current config already contains the requested sweep points
        if self.config().sweep_points == expected_sweep_points {
            return Ok(());
        }

        // Wait until the current config contains the requested sweep points
        info!("Waiting to receive updated config");
        let (lock, condvar) = &*self.device.config;
        let (_, timeout_result) = condvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |config| {
                    config
                        .filter(|config| config.sweep_points == expected_sweep_points)
                        .is_none()
                },
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
    #[tracing::instrument]
    pub fn set_calc_mode(&mut self, calc_mode: CalcMode) -> io::Result<()> {
        self.send_command(Command::SetCalcMode(calc_mode))
    }

    /// Sets the spectrum analyzer's input stage.
    #[tracing::instrument]
    pub fn set_input_stage(&mut self, input_stage: InputStage) -> io::Result<()> {
        self.send_command(Command::SetInputStage(input_stage))
    }

    /// Adds or subtracts an offset to the amplitudes in each sweep.
    #[tracing::instrument]
    pub fn set_offset_db(&mut self, offset_db: i8) -> io::Result<()> {
        self.send_command(Command::SetOffsetDB(offset_db))
    }

    /// Sets the spectrum analyzer's DSP mode.
    #[tracing::instrument]
    pub fn set_dsp_mode(&mut self, dsp_mode: DspMode) -> Result<()> {
        // Check to see if the DspMode is already set to the desired value
        if *self.device.dsp_mode.0.lock().unwrap() == Some(dsp_mode) {
            return Ok(());
        }

        // Send the command to set the DSP mode
        self.send_command(Command::SetDsp(dsp_mode))?;

        // Wait to see if we receive a DSP mode message in response
        let (lock, condvar) = &*self.device.dsp_mode;
        let (_, timeout_result) = condvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |new_dsp_mode| *new_dsp_mode != Some(dsp_mode),
            )
            .unwrap();

        if !timeout_result.timed_out() {
            Ok(())
        } else {
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
    }

    #[tracing::instrument]
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

    #[tracing::instrument]
    fn validate_min_max_amps(&self, min_amp_dbm: i16, max_amp_dbm: i16) -> Result<()> {
        // The bottom amplitude must be less than the top amplitude
        if min_amp_dbm >= max_amp_dbm {
            error!("");
            return Err(Error::InvalidInput(
                "The minimum amplitude must be less than the maximum amplitude".to_string(),
            ));
        }

        // The top and bottom amplitude must be within the RF Explorer's min and max amplitude range
        if !Self::MIN_MAX_AMP_RANGE_DBM.contains(&min_amp_dbm) {
            return Err(Error::InvalidInput(format!(
                "The amplitude {} dBm is not within the RF Explorer's amplitude range of {}-{} dBm",
                min_amp_dbm,
                Self::MIN_MAX_AMP_RANGE_DBM.start(),
                Self::MIN_MAX_AMP_RANGE_DBM.end()
            )));
        } else if !Self::MIN_MAX_AMP_RANGE_DBM.contains(&max_amp_dbm) {
            return Err(Error::InvalidInput(format!(
                "The amplitude {} dBm is not within the RF Explorer's amplitude range of {}-{} dBm",
                max_amp_dbm,
                Self::MIN_MAX_AMP_RANGE_DBM.start(),
                Self::MIN_MAX_AMP_RANGE_DBM.end()
            )));
        }

        Ok(())
    }
}
