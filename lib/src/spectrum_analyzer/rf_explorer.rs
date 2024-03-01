use std::{
    fmt::Debug,
    io,
    ops::RangeInclusive,
    sync::{Condvar, Mutex, MutexGuard, WaitTimeoutResult},
    time::Duration,
};

use tracing::{error, info, trace, warn};

use super::{
    CalcMode, Command, Config, DspMode, InputStage, Mode, Model, Sweep, TrackingStatus, WifiBand,
};
use crate::rf_explorer::{
    impl_rf_explorer, RadioModule, ScreenData, SerialNumber, SetupInfo, COMMAND_RESPONSE_TIMEOUT,
    NEXT_SCREEN_DATA_TIMEOUT, RECEIVE_INITIAL_DEVICE_INFO_TIMEOUT,
};
use crate::{
    common::{ConnectionError, ConnectionResult, Error, Frequency, Result},
    Device,
};

}

impl_rf_explorer!(SpectrumAnalyzer, MessageContainer);

impl SpectrumAnalyzer {
    const MIN_MAX_AMP_RANGE_DBM: RangeInclusive<i16> = -120..=35;
    const MIN_SWEEP_LEN: u16 = 112;
    const NEXT_SWEEP_TIMEOUT: Duration = Duration::from_secs(2);

    /// The serial number of the RF Explorer, if it exists.
    pub fn serial_number(&self) -> Option<String> {
        // Return the serial number if we've already received it
        if let Some(ref serial_number) = *self.messages().serial_number.0.lock().unwrap() {
            return Some(serial_number.to_string());
        }

        // If we haven't already received the serial number, request it from the RF Explorer
        self.send_command(crate::rf_explorer::Command::RequestSerialNumber)
            .ok()?;

        // Wait 2 seconds for the RF Explorer to send its serial number
        let (lock, cvar) = &self.messages().serial_number;
        tracing::trace!("Waiting to receive SerialNumber from RF Explorer");
        let _ = cvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                std::time::Duration::from_secs(2),
                |serial_number| serial_number.is_none(),
            )
            .unwrap();

        (*self.messages().serial_number.0.lock().unwrap())
            .as_ref()
            .map(|sn| sn.to_string())
    }

    /// The firmware version of the RF Explorer.
    pub fn firmware_version(&self) -> String {
        self.messages()
            .setup_info
            .0
            .lock()
            .unwrap()
            .as_ref()
            .map(|setup_info| setup_info.firmware_version.clone())
            .unwrap_or_default()
    }

    fn config(&self) -> MutexGuard<Option<Config>> {
        self.messages().config.0.lock().unwrap()
    }

    /// The start frequency of the RF Explorer's sweeps.
    pub fn start_freq(&self) -> Frequency {
        self.config()
            .as_ref()
            .map(|config| config.start_freq)
            .unwrap_or_default()
    }

    /// The step size of the RF Explorer's sweeps.
    pub fn step_size(&self) -> Frequency {
        self.config()
            .as_ref()
            .map(|config| config.step_size)
            .unwrap_or_default()
    }

    /// The stop frequency of the RF Explorer's sweeps.
    pub fn stop_freq(&self) -> Frequency {
        self.config()
            .as_ref()
            .map(|config| config.stop_freq)
            .unwrap_or_default()
    }

    /// The center frequency of the RF Explorer's sweeps.
    pub fn center_freq(&self) -> Frequency {
        self.config()
            .as_ref()
            .map(|config| config.center_freq)
            .unwrap_or_default()
    }

    /// The span of the RF Explorer's sweeps.
    pub fn span(&self) -> Frequency {
        self.config()
            .as_ref()
            .map(|config| config.span)
            .unwrap_or_default()
    }

    /// The minimum supported frequency of the RF Explorer.
    pub fn min_freq(&self) -> Frequency {
        self.config()
            .as_ref()
            .map(|config| config.min_freq)
            .unwrap_or_default()
    }

    /// The maximum supported frequency of the RF Explorer.
    pub fn max_freq(&self) -> Frequency {
        self.config()
            .as_ref()
            .map(|config| config.max_freq)
            .unwrap_or_default()
    }

    /// The maximum supported span of the RF Explorer.
    pub fn max_span(&self) -> Frequency {
        self.config()
            .as_ref()
            .map(|config| config.max_span)
            .unwrap_or_default()
    }

    /// The resolution bandwidth of the RF Explorer.
    pub fn rbw(&self) -> Option<Frequency> {
        self.config()
            .as_ref()
            .map(|config| config.rbw)
            .unwrap_or_default()
    }

    /// The minimum amplitude of sweeps displayed on the RF Explorer's screen.
    pub fn min_amp_dbm(&self) -> i16 {
        self.config()
            .as_ref()
            .map(|config| config.min_amp_dbm)
            .unwrap_or_default()
    }

    /// The maximum amplitude of sweeps displayed on the RF Explorer's screen.
    pub fn max_amp_dbm(&self) -> i16 {
        self.config()
            .as_ref()
            .map(|config| config.max_amp_dbm)
            .unwrap_or_default()
    }

    /// The amplitude offset of sweeps displayed on the RF Explorer's screen.
    pub fn amp_offset_db(&self) -> Option<i8> {
        self.config()
            .as_ref()
            .map(|config| config.amp_offset_db)
            .unwrap_or_default()
    }

    /// The number of amplitudes in the RF Explorer's sweeps.
    pub fn sweep_len(&self) -> u16 {
        self.config()
            .as_ref()
            .map(|config| config.sweep_len)
            .unwrap_or_default()
    }

    fn is_expansion_radio_module_active(&self) -> bool {
        self.config()
            .as_ref()
            .map(|config| config.is_expansion_radio_module_active)
            .unwrap_or_default()
    }

    /// The current `Mode` of the RF Explorer.
    pub fn mode(&self) -> Mode {
        self.config()
            .as_ref()
            .map(|config| config.mode)
            .unwrap_or_default()
    }

    /// The current `CalcMode` of the RF Explorer.
    pub fn calc_mode(&self) -> Option<CalcMode> {
        self.config()
            .as_ref()
            .map(|config| config.calc_mode)
            .unwrap_or_default()
    }

    /// The amplitudes of the most recent sweep measured by the RF Explorer.
    pub fn sweep(&self) -> Option<Vec<f32>> {
        self.rfe
            .messages()
            .sweep
            .0
            .lock()
            .unwrap()
            .as_ref()
            .map(|sweep| sweep.amplitudes_dbm.clone())
    }

    /// Fills the buffer with the amplitudes of the most recent sweep and returns the length of the sweep.
    pub fn fill_buf_with_sweep(&self, buf: &mut [f32]) -> Result<usize> {
        if let Some(sweep) = self.messages().sweep.0.lock().unwrap().as_ref() {
            let sweep_len = sweep.amplitudes_dbm.len();
            if buf.len() >= sweep_len {
                buf[0..sweep_len].copy_from_slice(sweep.amplitudes_dbm.as_slice());
                Ok(sweep_len)
            } else {
                Err(Error::InvalidInput(
                    "The buffer is too small to fit the sweep".to_string(),
                ))
            }
        } else {
            Err(Error::InvalidOperation(
                "No sweeps have been measured by the RF Explorer".to_string(),
            ))
        }
    }

    /// Waits for the RF Explorer to measure the next sweep.
    pub fn wait_for_next_sweep(&self) -> Result<Vec<f32>> {
        self.wait_for_next_sweep_with_timeout(Self::NEXT_SWEEP_TIMEOUT)
    }

    /// Waits for the RF Explorer to measure the next sweep and fills the buffer with its amplitudes.
    pub fn wait_for_next_sweep_and_fill_buf(&self, buf: &mut [f32]) -> Result<usize> {
        self.wait_for_next_sweep_with_timeout_and_fill_buf(Self::NEXT_SWEEP_TIMEOUT, buf)
    }

    /// Waits for the RF Explorer to measure the next sweep or for the timeout duration to elapse.
    pub fn wait_for_next_sweep_with_timeout(&self, timeout: Duration) -> Result<Vec<f32>> {
        let previous_sweep_timestamp = self
            .rfe
            .messages()
            .sweep
            .0
            .lock()
            .unwrap()
            .as_ref()
            .map(|sweep| sweep.timestamp);

        let (sweep, cond_var) = &self.messages().sweep;
        // Wait until the timestamp of the previous sweep and the next sweep are different
        let (sweep, wait_result) = cond_var
            .wait_timeout_while(sweep.lock().unwrap(), timeout, |sweep| {
                sweep.as_ref().map(|sweep| sweep.timestamp) == previous_sweep_timestamp
                    || sweep.is_none()
            })
            .unwrap();

        match &*sweep {
            Some(sweep) if !wait_result.timed_out() => Ok(sweep.amplitudes_dbm.clone()),
            _ => Err(Error::TimedOut(timeout)),
        }
    }

    /// Waits for the RF Explorer to measure the next sweep, or for the timeout duration to elapse,
    /// and fills the buffer with its amplitudes.
    pub fn wait_for_next_sweep_with_timeout_and_fill_buf(
        &self,
        timeout: Duration,
        buf: &mut [f32],
    ) -> Result<usize> {
        let previous_sweep_timestamp = self
            .rfe
            .messages()
            .sweep
            .0
            .lock()
            .unwrap()
            .as_ref()
            .map(|sweep| sweep.timestamp);

        let (sweep, cond_var) = &self.messages().sweep;
        // Wait until the timestamp of the previous sweep and the next sweep are different
        let (_, wait_result) = cond_var
            .wait_timeout_while(sweep.lock().unwrap(), timeout, |sweep| {
                sweep.as_ref().map(|sweep| sweep.timestamp) == previous_sweep_timestamp
                    || sweep.is_none()
            })
            .unwrap();

        if !wait_result.timed_out() {
            self.fill_buf_with_sweep(buf)
        } else {
            Err(Error::TimedOut(timeout))
        }
    }

    /// Returns the most recent `ScreenData` captured by the RF Explorer.
    pub fn screen_data(&self) -> Option<ScreenData> {
        self.messages().screen_data.0.lock().unwrap().clone()
    }

    /// Waits for the RF Explorer to capture its next `ScreenData`.
    pub fn wait_for_next_screen_data(&self) -> Result<ScreenData> {
        self.wait_for_next_screen_data_with_timeout(NEXT_SCREEN_DATA_TIMEOUT)
    }

    /// Waits for the RF Explorer to capture its next `ScreenData` or for the timeout duration to elapse.
    pub fn wait_for_next_screen_data_with_timeout(&self, timeout: Duration) -> Result<ScreenData> {
        let previous_screen_data = self.screen_data();

        let (screen_data, condvar) = &self.messages().screen_data;
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

    /// Returns the RF Explorer's DSP mode.
    pub fn dsp_mode(&self) -> Option<DspMode> {
        *self.messages().dsp_mode.0.lock().unwrap()
    }

    /// Returns the status of tracking mode (enabled or disabled).
    pub fn tracking_status(&self) -> Option<TrackingStatus> {
        *self.messages().tracking_status.0.lock().unwrap()
    }

    pub fn input_stage(&self) -> Option<InputStage> {
        *self.messages().input_stage.0.lock().unwrap()
    }

    /// Returns the main radio module.
    pub fn main_radio_module(&self) -> Option<RadioModule> {
        self.messages()
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
        self.rfe
            .messages()
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
        if self.is_expansion_radio_module_active() {
            self.expansion_radio_module().unwrap_or_default()
        } else {
            self.main_radio_module().unwrap_or_default()
        }
    }

    /// Returns the inactive radio module (if one exists).
    pub fn inactive_radio_module(&self) -> Option<RadioModule> {
        let expansion_radio_module = self.expansion_radio_module();
        if expansion_radio_module.is_some() {
            if self.is_expansion_radio_module_active() {
                self.main_radio_module()
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
        *self.messages().tracking_status.0.lock().unwrap() = None;

        // Send the command to enter tracking mode
        self.send_command(Command::StartTracking {
            start: Frequency::from_hz(start_hz),
            step: Frequency::from_hz(step_hz),
        })?;

        // Wait to see if we receive a tracking status message in response
        let (lock, condvar) = &self.messages().tracking_status;
        let (tracking_status, wait_result) = condvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                COMMAND_RESPONSE_TIMEOUT,
                |tracking_status| tracking_status.is_some(),
            )
            .unwrap();

        if !wait_result.timed_out() {
            Ok(tracking_status.unwrap_or_default())
        } else {
            Err(Error::TimedOut(COMMAND_RESPONSE_TIMEOUT))
        }
    }

    /// Steps over the tracking step frequency and makes a measurement.
    #[tracing::instrument(skip(self))]
    pub fn tracking_step(&self, step: u16) -> io::Result<()> {
        self.send_command(Command::TrackingStep(step))
    }

    /// Activates the RF Explorer's main radio module.
    pub fn activate_main_radio_module(&self) -> Result<()> {
        if self.active_radio_module().is_main() {
            return Err(Error::InvalidOperation(
                "Main radio module is already active.".to_string(),
            ));
        }

        self.send_command(Command::SwitchModuleMain)?;

        // Wait until config shows that the main radio module is active
        let _ = self.wait_for_config_while(|config| {
            config
                .as_ref()
                .filter(|config| !config.is_expansion_radio_module_active)
                .is_none()
        });

        if self.active_radio_module().is_main() {
            Ok(())
        } else {
            Err(Error::TimedOut(COMMAND_RESPONSE_TIMEOUT))
        }
    }

    /// Activates the RF Explorer's expansion radio module (if one exists).
    pub fn activate_expansion_radio_module(&self) -> Result<()> {
        if self.expansion_radio_module().is_none() {
            return Err(Error::InvalidOperation(
                "This RF Explorer does not contain an expansion radio module.".to_string(),
            ));
        }

        if self.active_radio_module().is_expansion() {
            return Err(Error::InvalidOperation(
                "Expansion radio module is already active.".to_string(),
            ));
        }

        self.send_command(Command::SwitchModuleExp)?;

        // Wait until config shows that the expansion radio module is active
        let _ = self.wait_for_config_while(|config| {
            config
                .as_ref()
                .filter(|config| config.is_expansion_radio_module_active)
                .is_none()
        });

        if self.active_radio_module().is_expansion() {
            Ok(())
        } else {
            Err(Error::TimedOut(COMMAND_RESPONSE_TIMEOUT))
        }
    }

    /// Sets the start and stop frequency of sweeps measured by the spectrum analyzer.
    pub fn set_start_stop(
        &self,
        start: impl Into<Frequency>,
        stop: impl Into<Frequency>,
    ) -> Result<()> {
        self.set_config(
            start.into(),
            stop.into(),
            self.min_amp_dbm(),
            self.max_amp_dbm(),
        )
    }

    /// Sets the start frequency, stop frequency, and number of points of sweeps measured by the spectrum analyzer.
    pub fn set_start_stop_sweep_len(
        &self,
        start: impl Into<Frequency>,
        stop: impl Into<Frequency>,
        sweep_len: u16,
    ) -> Result<()> {
        self.set_sweep_len(sweep_len)?;
        self.set_start_stop(start, stop)
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
    pub fn set_center_span_sweep_len(
        &self,
        center: impl Into<Frequency>,
        span: impl Into<Frequency>,
        sweep_len: u16,
    ) -> Result<()> {
        let (center, span) = (center.into(), span.into());
        self.set_start_stop_sweep_len(center - span / 2, center + span / 2, sweep_len)
    }

    /// Sets the minimum and maximum amplitudes displayed on the RF Explorer's screen.
    #[tracing::instrument(skip(self))]
    pub fn set_min_max_amps(&self, min_amp_dbm: i16, max_amp_dbm: i16) -> Result<()> {
        self.set_config(
            self.start_freq(),
            self.stop_freq(),
            min_amp_dbm,
            max_amp_dbm,
        )
    }

    /// Sets the spectrum analyzer's configuration.
    #[tracing::instrument(skip(self), ret, err)]
    fn set_config(
        &self,
        start: Frequency,
        stop: Frequency,
        min_amp_dbm: i16,
        max_amp_dbm: i16,
    ) -> Result<()> {
        self.validate_start_stop(start, stop)?;
        self.validate_min_max_amps(min_amp_dbm, max_amp_dbm)?;

        self.send_command(Command::SetConfig {
            start,
            stop,
            min_amp_dbm,
            max_amp_dbm,
        })?;

        // Check if the current config already contains the requested values
        if self
            .config()
            .as_ref()
            .unwrap_or(&Config::default())
            .contains_start_stop_amp_range(start, stop, min_amp_dbm, max_amp_dbm)
        {
            return Ok(());
        }

        // Wait until the current config contains the requested values
        trace!("Waiting to receive updated 'Config'");
        let (_, wait_result) = self.wait_for_config_while(|config| {
            let Some(config) = config else {
                return true;
            };

            !config.contains_start_stop_amp_range(start, stop, min_amp_dbm, max_amp_dbm)
        });

        if !wait_result.timed_out() {
            Ok(())
        } else {
            Err(Error::TimedOut(COMMAND_RESPONSE_TIMEOUT))
        }
    }

    /// Sets the callback that is called when the spectrum analyzer receives a sweep.
    pub fn set_sweep_callback(&self, cb: impl FnMut(&[f32]) + Send + 'static) {
        *self.messages().sweep_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Removes the callback that is called when the spectrum analyzer receives a `Sweep`.
    pub fn remove_sweep_callback(&self) {
        *self.messages().sweep_callback.lock().unwrap() = None;
    }

    /// Sets the callback that is called when the spectrum analyzer receives a `Config`.
    pub fn set_config_callback(&self, cb: impl FnMut() + Send + 'static) {
        *self.messages().config_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Removes the callback that is called when the spectrum analyzer receives a `Config`.
    pub fn remove_config_callback(&self) {
        *self.messages().config_callback.lock().unwrap() = None;
    }

    /// Sets the number of points in each sweep measured by the spectrum analyzer.
    #[tracing::instrument(skip(self))]
    pub fn set_sweep_len(&self, sweep_len: u16) -> Result<()> {
        // Only 'Plus' models can set the number of points in a sweep
        if !self.active_radio_module().model().is_plus_model() {
            return Err(Error::InvalidOperation(
                "Only RF Explorer 'Plus' models support setting the number of sweep points"
                    .to_string(),
            ));
        }

        if sweep_len <= 4096 {
            self.send_command(Command::SetSweepPointsExt(sweep_len))?;
        } else {
            self.send_command(Command::SetSweepPointsLarge(sweep_len))?;
        }

        // The requested number of sweep points gets rounded down to a number that's a multiple of 16
        let expected_sweep_len = if sweep_len < 112 {
            Self::MIN_SWEEP_LEN
        } else {
            (sweep_len / 16) * 16
        };

        // Check if the current config already contains the requested sweep points
        if self.sweep_len() == expected_sweep_len {
            return Ok(());
        }

        // Wait until the current config contains the requested sweep points
        info!("Waiting to receive updated config");
        let (_, wait_result) = self.wait_for_config_while(|config| {
            config
                .as_ref()
                .filter(|config| config.sweep_len == expected_sweep_len)
                .is_none()
        });

        if !wait_result.timed_out() {
            Ok(())
        } else {
            warn!("Failed to receive updated config");
            Err(Error::TimedOut(COMMAND_RESPONSE_TIMEOUT))
        }
    }

    /// Sets the spectrum analyzer's calculator mode.
    #[tracing::instrument(skip(self))]
    pub fn set_calc_mode(&self, calc_mode: CalcMode) -> io::Result<()> {
        self.send_command(Command::SetCalcMode(calc_mode))
    }

    /// Sets the spectrum analyzer's input stage.
    #[tracing::instrument(skip(self))]
    pub fn set_input_stage(&self, input_stage: InputStage) -> io::Result<()> {
        self.send_command(Command::SetInputStage(input_stage))
    }

    /// Adds or subtracts an offset to the amplitudes in each sweep.
    #[tracing::instrument(skip(self))]
    pub fn set_offset_db(&self, offset_db: i8) -> io::Result<()> {
        self.send_command(Command::SetOffsetDB(offset_db))
    }

    /// Sets the spectrum analyzer's DSP mode.
    #[tracing::instrument(skip(self))]
    pub fn set_dsp_mode(&self, dsp_mode: DspMode) -> Result<()> {
        // Check to see if the DspMode is already set to the desired value
        if *self.messages().dsp_mode.0.lock().unwrap() == Some(dsp_mode) {
            return Ok(());
        }

        // Send the command to set the DSP mode
        self.send_command(Command::SetDsp(dsp_mode))?;

        // Wait to see if we receive a DSP mode message in response
        let (lock, condvar) = &self.messages().dsp_mode;
        let (_, wait_result) = condvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                COMMAND_RESPONSE_TIMEOUT,
                |new_dsp_mode| *new_dsp_mode != Some(dsp_mode),
            )
            .unwrap();

        if !wait_result.timed_out() {
            Ok(())
        } else {
            Err(Error::TimedOut(COMMAND_RESPONSE_TIMEOUT))
        }
    }

    fn wait_for_config_while(
        &self,
        condition: impl FnMut(&mut Option<Config>) -> bool,
    ) -> (MutexGuard<Option<Config>>, WaitTimeoutResult) {
        let (lock, condvar) = &self.messages().config;
        condvar
            .wait_timeout_while(lock.lock().unwrap(), COMMAND_RESPONSE_TIMEOUT, condition)
            .unwrap()
    }

    #[tracing::instrument(skip(self), ret, err)]
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

    #[tracing::instrument(skip(self), ret, err)]
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

#[derive(Default)]
struct MessageContainer {
    pub(crate) config: (Mutex<Option<Config>>, Condvar),
    pub(crate) config_callback: Mutex<Option<Box<dyn FnMut() + Send>>>,
    pub(crate) sweep: (Mutex<Option<Sweep>>, Condvar),
    pub(crate) sweep_callback: Mutex<Option<Box<dyn FnMut(&[f32]) + Send>>>,
    pub(crate) screen_data: (Mutex<Option<ScreenData>>, Condvar),
    pub(crate) dsp_mode: (Mutex<Option<DspMode>>, Condvar),
    pub(crate) tracking_status: (Mutex<Option<TrackingStatus>>, Condvar),
    pub(crate) input_stage: (Mutex<Option<InputStage>>, Condvar),
    pub(crate) setup_info: (Mutex<Option<SetupInfo>>, Condvar),
    pub(crate) serial_number: (Mutex<Option<SerialNumber>>, Condvar),
}

impl crate::common::MessageContainer for MessageContainer {
    type Message = super::Message;

    fn cache_message(&self, message: Self::Message) {
        match message {
            Self::Message::Config(config) => {
                *self.config.0.lock().unwrap() = Some(config);
                self.config.1.notify_one();
                if let Some(ref mut cb) = *self.config_callback.lock().unwrap() {
                    cb();
                }
            }
            Self::Message::Sweep(sweep) => {
                *self.sweep.0.lock().unwrap() = Some(sweep);
                self.sweep.1.notify_one();
                if let Some(ref mut cb) = *self.sweep_callback.lock().unwrap() {
                    if let Some(ref sweep) = *self.sweep.0.lock().unwrap() {
                        cb(sweep.amplitudes_dbm.as_slice());
                    }
                }
            }
            Self::Message::ScreenData(screen_data) => {
                *self.screen_data.0.lock().unwrap() = Some(screen_data);
                self.screen_data.1.notify_one();
            }
            Self::Message::DspMode(dsp_mode) => {
                *self.dsp_mode.0.lock().unwrap() = Some(dsp_mode);
                self.dsp_mode.1.notify_one();
            }
            Self::Message::InputStage(input_stage) => {
                *self.input_stage.0.lock().unwrap() = Some(input_stage);
                self.input_stage.1.notify_one();
            }
            Self::Message::TrackingStatus(tracking_status) => {
                *self.tracking_status.0.lock().unwrap() = Some(tracking_status);
                self.tracking_status.1.notify_one();
            }
            Self::Message::SerialNumber(serial_number) => {
                *self.serial_number.0.lock().unwrap() = Some(serial_number);
                self.serial_number.1.notify_one();
            }
            Self::Message::SetupInfo(setup_info) => {
                *self.setup_info.0.lock().unwrap() = Some(setup_info);
                self.setup_info.1.notify_one();
            }
        }
    }

    fn wait_for_device_info(&self) -> ConnectionResult<()> {
        let (config_lock, config_cvar) = &self.config;
        let (setup_info_lock, setup_info_cvar) = &self.setup_info;

        // Check to see if we've already received a Config and SetupInfo
        if config_lock.lock().unwrap().is_some() && setup_info_lock.lock().unwrap().is_some() {
            return Ok(());
        }

        // Wait to see if we receive a Config and SetupInfo before timing out
        if config_cvar
            .wait_timeout_while(
                config_lock.lock().unwrap(),
                RECEIVE_INITIAL_DEVICE_INFO_TIMEOUT,
                |config| config.is_none(),
            )
            .unwrap()
            .0
            .is_some()
            && setup_info_cvar
                .wait_timeout_while(
                    setup_info_lock.lock().unwrap(),
                    RECEIVE_INITIAL_DEVICE_INFO_TIMEOUT,
                    |setup_info| setup_info.is_none(),
                )
                .unwrap()
                .0
                .is_some()
        {
            Ok(())
        } else {
            Err(ConnectionError::DeviceInfoNotReceived)
        }
    }
}

impl Debug for MessageContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageContainer")
            .field("config", &self.config.0.lock().unwrap())
            .field("sweep", &self.sweep.0.lock().unwrap())
            .field("screen_data", &self.screen_data.0.lock().unwrap())
            .field("dsp_mode", &self.dsp_mode.0.lock().unwrap())
            .field("tracking_status", &self.tracking_status.0.lock().unwrap())
            .field("input_stage", &self.input_stage.0.lock().unwrap())
            .field("setup_info", &self.setup_info.0.lock().unwrap())
            .field("serial_number", &self.serial_number.0.lock().unwrap())
            .finish()
    }
}
