use std::{io, time::Duration};

use super::{
    Attenuation, Command, Config, ConfigAmpSweep, ConfigCw, ConfigFreqSweep, Message, PowerLevel,
    Temperature,
};
use crate::common::{Error, Frequency, RadioModule, Result, RfExplorer, ScreenData};

impl RfExplorer<SignalGenerator> {
    /// Returns the signal generator's configuration.
    pub fn config(&self) -> Config {
        self.device.config.0.lock().unwrap().unwrap_or_default()
    }

    /// Returns the signal generator's amplitude sweep mode configuration.
    pub fn config_amp_sweep(&self) -> Option<ConfigAmpSweep> {
        *self.device.config_amp_sweep.0.lock().unwrap()
    }

    /// Returns the signal generator's CW mode configuration.
    pub fn config_cw(&self) -> Option<ConfigCw> {
        *self.device.config_cw.0.lock().unwrap()
    }

    /// Returns the signal generator's frequency sweep mode configuration.
    pub fn config_freq_sweep(&self) -> Option<ConfigFreqSweep> {
        *self.device.config_freq_sweep.0.lock().unwrap()
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

    /// Returns the signal generator's temperature.
    pub fn temperature(&self) -> Option<Temperature> {
        *self.device.temperature.0.lock().unwrap()
    }

    /// Returns the main radio module.
    pub fn main_radio_module(&self) -> RadioModule<Model> {
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
    pub fn expansion_radio_module(&self) -> Option<RadioModule<Model>> {
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
    pub fn active_radio_module(&self) -> RadioModule<Model> {
        let Some(exp_module) = self.expansion_radio_module() else {
            return self.main_radio_module();
        };

        if self.config_expansion().is_some() {
            exp_module
        } else {
            self.main_radio_module()
        }
    }

    /// Returns the inactive radio module (if one exists).
    pub fn inactive_radio_module(&self) -> Option<RadioModule<Model>> {
        let Some(exp_module) = self.expansion_radio_module() else {
            return None;
        };

        if self.config_expansion().is_some() {
            Some(self.main_radio_module())
        } else {
            Some(exp_module)
        }
    }

    /// Starts the signal generator's amplitude sweep mode.
    pub fn start_amp_sweep(
        &self,
        cw: impl Into<Frequency>,
        start_attenuation: Attenuation,
        start_power_level: PowerLevel,
        stop_attenuation: Attenuation,
        stop_power_level: PowerLevel,
        step_delay: Duration,
    ) -> io::Result<()> {
        self.send_command(Command::StartAmpSweep {
            cw: cw.into(),
            start_attenuation,
            start_power_level,
            stop_attenuation,
            stop_power_level,
            step_delay,
        })
    }

    /// Starts the signal generator's amplitude sweep mode using the expansion module.
    pub fn start_amp_sweep_exp(
        &self,
        cw: impl Into<Frequency>,
        start_power_dbm: f64,
        step_power_db: f64,
        stop_power_dbm: f64,
        step_delay: Duration,
    ) -> io::Result<()> {
        self.send_command(Command::StartAmpSweepExp {
            cw: cw.into(),
            start_power_dbm,
            step_power_db,
            stop_power_dbm,
            step_delay,
        })
    }

    /// Starts the signal generator's CW mode.
    pub fn start_cw(
        &self,
        cw: impl Into<Frequency>,
        attenuation: Attenuation,
        power_level: PowerLevel,
    ) -> io::Result<()> {
        self.send_command(Command::StartCw {
            cw: cw.into(),
            attenuation,
            power_level,
        })
    }

    /// Starts the signal generator's CW mode using the expansion module.
    pub fn start_cw_exp(&self, cw: impl Into<Frequency>, power_dbm: f64) -> io::Result<()> {
        self.send_command(Command::StartCwExp {
            cw: cw.into(),
            power_dbm,
        })
    }

    /// Starts the signal generator's frequency sweep mode.
    pub fn start_freq_sweep(
        &self,
        start: impl Into<Frequency>,
        attenuation: Attenuation,
        power_level: PowerLevel,
        sweep_steps: u16,
        step_hz: u64,
        step_delay: Duration,
    ) -> io::Result<()> {
        self.send_command(Command::StartFreqSweep {
            start: start.into(),
            attenuation,
            power_level,
            sweep_steps,
            step: Frequency::from_hz(step_hz),
            step_delay,
        })
    }

    /// Starts the signal generator's frequency sweep mode using the expansion module.
    pub fn start_freq_sweep_exp(
        &self,
        start: impl Into<Frequency>,
        power_dbm: f64,
        sweep_steps: u16,
        step: impl Into<Frequency>,
        step_delay: Duration,
    ) -> io::Result<()> {
        self.send_command(Command::StartFreqSweepExp {
            start: start.into(),
            power_dbm,
            sweep_steps,
            step: step.into(),
            step_delay,
        })
    }

    /// Starts the signal generator's tracking mode.
    pub fn start_tracking(
        &self,
        start: impl Into<Frequency>,
        attenuation: Attenuation,
        power_level: PowerLevel,
        sweep_steps: u16,
        step: impl Into<Frequency>,
    ) -> io::Result<()> {
        self.send_command(Command::StartTracking {
            start: start.into(),
            attenuation,
            power_level,
            sweep_steps,
            step: step.into(),
        })
    }

    /// Starts the signal generator's tracking mode using the expansion module.
    pub fn start_tracking_exp(
        &self,
        start: impl Into<Frequency>,
        power_dbm: f64,
        sweep_steps: u16,
        step: impl Into<Frequency>,
    ) -> io::Result<()> {
        self.send_command(Command::StartTrackingExp {
            start: start.into(),
            power_dbm,
            sweep_steps,
            step: step.into(),
        })
    }

    /// Jumps to a new frequency using the tracking step frequency.
    pub fn tracking_step(&self, steps: u16) -> io::Result<()> {
        self.send_command(Command::TrackingStep(steps))
    }

    /// Sets the callback that is called when the signal generator receives a `Config`.
    pub fn set_config_callback(&self, cb: impl FnMut(Config) + Send + 'static) {
        *self.device.config_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Sets the callback that is called when the signal generator receives a `ConfigAmpSweep`.
    pub fn set_config_amp_sweep_callback(&self, cb: impl FnMut(ConfigAmpSweep) + Send + 'static) {
        *self.device.config_amp_sweep_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Sets the callback that is called when the signal generator receives a `ConfigCw`.
    pub fn set_config_cw_callback(&self, cb: impl FnMut(ConfigCw) + Send + 'static) {
        *self.device.config_cw_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Sets the callback that is called when the signal generator receives a `ConfigFreqSweep`.
    pub fn set_config_freq_sweep_callback(&self, cb: impl FnMut(ConfigFreqSweep) + Send + 'static) {
        *self.device.config_freq_sweep_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Turns on RF power with the current power and frequency configuration.
    pub fn rf_power_on(&self) -> io::Result<()> {
        self.send_command(Command::RfPowerOn)
    }

    /// Turns off RF power.
    pub fn rf_power_off(&self) -> io::Result<()> {
        self.send_command(Command::RfPowerOff)
    }
}
