use std::{
    fmt::Debug,
    io,
    ops::RangeInclusive,
    sync::{MutexGuard, WaitTimeoutResult},
    time::Duration,
};

use num_enum::IntoPrimitive;
use tracing::{error, info, warn};

use super::{
    CalcMode, Command, Config, DspMode, InputStage, SpectrumAnalyzer, Sweep, TrackingStatus,
};
use crate::common::{Device, Error, Frequency, RadioModule, Result, RfExplorer, ScreenData};

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive)]
#[repr(u8)]
pub enum WifiBand {
    TwoPointFourGhz = 1,
    FiveGhz,
}

impl RfExplorer<SpectrumAnalyzer> {
    const MIN_MAX_AMP_RANGE_DBM: RangeInclusive<i16> = -120..=35;
    const MIN_SWEEP_POINTS: u16 = 112;
    const NEXT_SWEEP_TIMEOUT: Duration = Duration::from_secs(2);

    /// Returns the RF Explorer's current `Config`.
    pub fn config(&self) -> Config {
        self.device.config.0.lock().unwrap().unwrap_or_default()
    }

    /// Returns the most recent `Sweep` measured by the RF Explorer.
    pub fn sweep(&self) -> Option<Sweep> {
        self.device.sweep.0.lock().unwrap().clone()
    }

    /// Waits for the RF Explorer to measure its next `Sweep`.
    pub fn wait_for_next_sweep(&self) -> Result<Sweep> {
        self.wait_for_next_sweep_with_timeout(Self::NEXT_SWEEP_TIMEOUT)
    }

    /// Waits for the RF Explorer to measure its next `Sweep` or for the timeout duration to elapse.
    pub fn wait_for_next_sweep_with_timeout(&self, timeout: Duration) -> Result<Sweep> {
        let previous_sweep = self.sweep();

        let (sweep, cond_var) = &*self.device.sweep;
        let (sweep, wait_result) = cond_var
            .wait_timeout_while(sweep.lock().unwrap(), timeout, |sweep| {
                *sweep == previous_sweep || sweep.is_none()
            })
            .unwrap();

        match &*sweep {
            Some(sweep) if !wait_result.timed_out() => Ok(sweep.clone()),
            _ => Err(Error::TimedOut(timeout)),
        }
    }

    /// Returns the most recent `ScreenData` captured by the RF Explorer.
    pub fn screen_data(&self) -> Option<ScreenData> {
        self.device.screen_data.0.lock().unwrap().clone()
    }

    /// Waits for the RF Explorer to capture its next `ScreenData`.
    pub fn wait_for_next_screen_data(&self) -> Result<ScreenData> {
        self.wait_for_next_screen_data_with_timeout(Self::NEXT_SCREEN_DATA_TIMEOUT)
    }

    /// Waits for the RF Explorer to capture its next `ScreenData` or for the timeout duration to elapse.
    pub fn wait_for_next_screen_data_with_timeout(&self, timeout: Duration) -> Result<ScreenData> {
        let previous_screen_data = self.screen_data();

        let (screen_data, condvar) = &*self.device.screen_data;
        let (screen_data, wait_result) = condvar
            .wait_timeout_while(screen_data.lock().unwrap(), timeout, |screen_data| {
                *screen_data == previous_screen_data || screen_data.is_none()
            })
            .unwrap();

        match &*screen_data {
            Some(screen_data) if !wait_result.timed_out() => Ok(screen_data.clone()),
            _ => Err(Error::TimedOut(timeout)),
        }
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

    /// Returns the main radio module.
    pub fn main_radio_module(&self) -> RadioModule {
        self.device
            .setup_info
            .0
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .main_radio_module
    }

    /// Returns the expansion radio module (if one exists).
    pub fn expansion_radio_module(&self) -> Option<RadioModule> {
        self.device
            .setup_info
            .0
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .expansion_radio_module
    }

    /// Returns the active radio module.
    pub fn active_radio_module(&self) -> RadioModule {
        if self.config().is_expansion_radio_module_active {
            self.expansion_radio_module().unwrap()
        } else {
            self.main_radio_module()
        }
    }

    /// Returns the inactive radio module (if one exists).
    pub fn inactive_radio_module(&self) -> Option<RadioModule> {
        let expansion_radio_module = self.expansion_radio_module();
        if expansion_radio_module.is_some() {
            if self.config().is_expansion_radio_module_active {
                Some(self.main_radio_module())
            } else {
                expansion_radio_module
            }
        } else {
            None
        }
    }

    /// Starts the spectrum analyzer's Wi-Fi analyzer.
    #[tracing::instrument]
    pub fn start_wifi_analyzer(&self, wifi_band: WifiBand) -> io::Result<()> {
        self.send_command(Command::StartWifiAnalyzer(wifi_band))
    }

    /// Stops the spectrum analyzer's Wi-Fi analyzer.
    #[tracing::instrument(skip(self))]
    pub fn stop_wifi_analyzer(&self) -> io::Result<()> {
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
            start: Frequency::from_hz(start_hz),
            step: Frequency::from_hz(step_hz),
        })?;

        // Wait to see if we receive a tracking status message in response
        let (lock, condvar) = &*self.device.tracking_status;
        let (tracking_status, wait_result) = condvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |tracking_status| tracking_status.is_some(),
            )
            .unwrap();

        if !wait_result.timed_out() {
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
        start: impl Into<Frequency>,
        stop: impl Into<Frequency>,
    ) -> Result<()> {
        let config = self.config();
        self.set_config(
            start.into(),
            stop.into(),
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
        center: impl Into<Frequency>,
        span: impl Into<Frequency>,
    ) -> Result<()> {
        let (center, span) = (center.into(), span.into());
        self.set_start_stop(center - span / 2, center + span / 2)
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
        self.set_config(config.start, config.stop, min_amp_dbm, max_amp_dbm)
    }

    /// Sets the spectrum analyzer's configuration.
    #[tracing::instrument(skip(self))]
    fn set_config(
        &self,
        start: Frequency,
        stop: Frequency,
        min_amp_dbm: i16,
        max_amp_dbm: i16,
    ) -> Result<()> {
        info!("Validating start and stop frequencies");
        self.validate_start_stop(start, stop)?;
        info!("Validating min and max amplitudes");
        self.validate_min_max_amps(min_amp_dbm, max_amp_dbm)?;

        // Send the command to change the config
        info!("Sending 'SetConfig' command");
        self.send_command(Command::SetConfig {
            start,
            stop,
            min_amp_dbm,
            max_amp_dbm,
        })?;

        // Function to check whether a config contains the requested values
        let config_contains_requested_values = |config: &Config| {
            config.start.abs_diff(start) < config.step
                && config.stop.abs_diff(stop) < config.step
                && config.min_amp_dbm == min_amp_dbm
                && config.max_amp_dbm == max_amp_dbm
        };

        // Check if the current config already contains the requested values
        if config_contains_requested_values(&self.config()) {
            return Ok(());
        }

        // Wait until the current config contains the requested values
        info!("Waiting to receive updated config");
        let (_, wait_result) = self.wait_for_config_while(|config| {
            config.filter(config_contains_requested_values).is_none()
        });

        if !wait_result.timed_out() {
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
        if !self.active_radio_module().model().is_plus_model() {
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
        let (_, wait_result) = self.wait_for_config_while(|config| {
            config
                .filter(|config| config.sweep_points == expected_sweep_points)
                .is_none()
        });

        if !wait_result.timed_out() {
            Ok(())
        } else {
            warn!("Failed to receive updated config");
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
    }

    /// Sets the spectrum analyzer's calculator mode.
    #[tracing::instrument]
    pub fn set_calc_mode(&self, calc_mode: CalcMode) -> io::Result<()> {
        self.send_command(Command::SetCalcMode(calc_mode))
    }

    /// Sets the spectrum analyzer's active radio module
    pub fn set_active_radio_module(&self, radio_module: RadioModule) -> Result<()> {
        // Check if the RF Explorer has more than one radio module to switch between
        if self.inactive_module().is_none() {
            return Err(Error::InvalidOperation(format!(
                "This RF Explorer only has 1 possible radio module: ({})",
                self.active_module_model()
            )));
        }

        // Check if the given radio module is already active
        if self.config().active_radio_module == radio_module {
            return Ok(());
        }

        match radio_module {
            RadioModule::Main => self.send_command(Command::SwitchModuleMain)?,
            RadioModule::Expansion => self.send_command(Command::SwitchModuleExp)?,
        }

        // Wait until the config shows that the given module is active
        let (_, wait_result) = self.wait_for_config_while(|config| {
            config
                .filter(|config| config.active_radio_module == radio_module)
                .is_none()
        });

        if !wait_result.timed_out() {
            Ok(())
        } else {
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
    }

    /// Sets the spectrum analyzer's input stage.
    #[tracing::instrument]
    pub fn set_input_stage(&self, input_stage: InputStage) -> io::Result<()> {
        self.send_command(Command::SetInputStage(input_stage))
    }

    /// Adds or subtracts an offset to the amplitudes in each sweep.
    #[tracing::instrument]
    pub fn set_offset_db(&self, offset_db: i8) -> io::Result<()> {
        self.send_command(Command::SetOffsetDB(offset_db))
    }

    /// Sets the spectrum analyzer's DSP mode.
    #[tracing::instrument]
    pub fn set_dsp_mode(&self, dsp_mode: DspMode) -> Result<()> {
        // Check to see if the DspMode is already set to the desired value
        if *self.device.dsp_mode.0.lock().unwrap() == Some(dsp_mode) {
            return Ok(());
        }

        // Send the command to set the DSP mode
        self.send_command(Command::SetDsp(dsp_mode))?;

        // Wait to see if we receive a DSP mode message in response
        let (lock, condvar) = &*self.device.dsp_mode;
        let (_, wait_result) = condvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                |new_dsp_mode| *new_dsp_mode != Some(dsp_mode),
            )
            .unwrap();

        if !wait_result.timed_out() {
            Ok(())
        } else {
            Err(Error::TimedOut(SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT))
        }
    }

    fn wait_for_config_while(
        &self,
        condition: impl FnMut(&mut Option<Config>) -> bool,
    ) -> (MutexGuard<Option<Config>>, WaitTimeoutResult) {
        let (lock, condvar) = &*self.device.config;
        condvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::COMMAND_RESPONSE_TIMEOUT,
                condition,
            )
            .unwrap()
    }

    #[tracing::instrument]
    fn validate_start_stop(&self, start: Frequency, stop: Frequency) -> Result<()> {
        if start >= stop {
            return Err(Error::InvalidInput(
                "The start frequency must be less than the stop frequency".to_string(),
            ));
        }

        let active_model = self.active_radio_module().model();

        let min_max_freq = active_model.min_freq()..=active_model.max_freq();
        if !min_max_freq.contains(&start) {
            return Err(Error::InvalidInput(format!(
                "The start frequency {} MHz is not within the RF Explorer's frequency range of {}-{} MHz",
                start.as_mhz_f64(),
                min_max_freq.start().as_mhz_f64(),
                min_max_freq.end().as_mhz_f64()
            )));
        } else if !min_max_freq.contains(&stop) {
            return Err(Error::InvalidInput(format!(
                "The stop frequency {} MHz is not within the RF Explorer's frequency range of {}-{} MHz",
                stop.as_mhz(),
                min_max_freq.start().as_mhz_f64(),
                min_max_freq.end().as_mhz_f64()
            )));
        }

        let min_max_span = active_model.min_span()..=active_model.max_span();
        if !min_max_span.contains(&(stop - start)) {
            return Err(Error::InvalidInput(format!(
                "The span {} MHz is not within the RF Explorer's span range of {}-{} MHz",
                (stop - start).as_mhz_f64(),
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
