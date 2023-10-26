use std::{
    fmt::Debug,
    io,
    ops::Deref,
    sync::{Condvar, Mutex},
    time::Duration,
};

use super::{
    Attenuation, Config, ConfigAmpSweep, ConfigAmpSweepExp, ConfigCw, ConfigCwExp, ConfigExp,
    ConfigFreqSweep, ConfigFreqSweepExp, Model, PowerLevel, Temperature,
};
use crate::common::{ConnectionError, ConnectionResult, Frequency};
use crate::rf_explorer::{
    Callback, RadioModule, RfExplorer, RfExplorerMessageContainer, ScreenData, SerialNumber,
    SetupInfo,
};

#[derive(Debug)]
pub struct SignalGenerator {
    rfe: RfExplorer<MessageContainer>,
}

impl SignalGenerator {
    pub fn connect() -> Option<Self> {
        Some(Self {
            rfe: RfExplorer::connect()?,
        })
    }

    pub fn connect_with_name_and_baud_rate(name: &str, baud_rate: u32) -> ConnectionResult<Self> {
        Ok(Self {
            rfe: RfExplorer::connect_with_name_and_baud_rate(name, baud_rate)?,
        })
    }

    pub fn connect_all() -> Vec<Self> {
        RfExplorer::connect_all()
            .into_iter()
            .map(|rfe| Self { rfe })
            .collect()
    }

    pub fn serial_number(&self) -> io::Result<crate::SerialNumber> {
        if let Some(ref serial_number) = *self.message_container().serial_number.0.lock().unwrap() {
            return Ok(serial_number.clone());
        }

        self.send_command(crate::rf_explorer::Command::RequestSerialNumber)?;

        let (lock, cvar) = &self.message_container().serial_number;
        tracing::trace!("Waiting to receive SerialNumber from RF Explorer");
        let _ = cvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                std::time::Duration::from_secs(2),
                |serial_number| serial_number.is_none(),
            )
            .unwrap();

        if let Some(ref serial_number) = *self.message_container().serial_number.0.lock().unwrap() {
            Ok(serial_number.clone())
        } else {
            Err(io::ErrorKind::TimedOut.into())
        }
    }

    pub fn config(&self) -> Option<Config> {
        *self.message_container().config.0.lock().unwrap()
    }

    pub fn config_expansion(&self) -> Option<ConfigExp> {
        *self.message_container().config_exp.0.lock().unwrap()
    }

    pub fn config_amp_sweep(&self) -> Option<ConfigAmpSweep> {
        *self.message_container().config_amp_sweep.0.lock().unwrap()
    }

    pub fn config_amp_sweep_expansion(&self) -> Option<ConfigAmpSweepExp> {
        *self
            .message_container()
            .config_amp_sweep_exp
            .0
            .lock()
            .unwrap()
    }

    pub fn config_cw(&self) -> Option<ConfigCw> {
        *self.message_container().config_cw.0.lock().unwrap()
    }

    pub fn config_cw_expansion(&self) -> Option<ConfigCwExp> {
        *self.message_container().config_cw_exp.0.lock().unwrap()
    }

    pub fn config_freq_sweep(&self) -> Option<ConfigFreqSweep> {
        *self.message_container().config_freq_sweep.0.lock().unwrap()
    }

    pub fn config_freq_sweep_expansion(&self) -> Option<ConfigFreqSweepExp> {
        *self
            .message_container()
            .config_freq_sweep_exp
            .0
            .lock()
            .unwrap()
    }

    pub fn wait_for_next_screen_data(&self) -> crate::Result<ScreenData> {
        self.wait_for_next_screen_data_with_timeout(
            RfExplorer::<MessageContainer>::NEXT_SCREEN_DATA_TIMEOUT,
        )
    }

    pub fn wait_for_next_screen_data_with_timeout(
        &self,
        timeout: Duration,
    ) -> crate::Result<ScreenData> {
        let previous_screen_data = self.screen_data();
        let (screen_data, condvar) = &self.message_container().screen_data;
        let (screen_data, wait_result) = condvar
            .wait_timeout_while(screen_data.lock().unwrap(), timeout, |screen_data| {
                *screen_data == previous_screen_data || screen_data.is_none()
            })
            .unwrap();

        match &*screen_data {
            Some(screen_data) if !wait_result.timed_out() => Ok(screen_data.clone()),
            _ => Err(crate::Error::TimedOut(timeout)),
        }
    }

    pub fn temperature(&self) -> Option<Temperature> {
        *self.message_container().temperature.0.lock().unwrap()
    }

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
        self.send_command(super::Command::StartAmpSweep {
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
        self.send_command(super::Command::StartAmpSweepExp {
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
        self.send_command(super::Command::StartCw {
            cw: cw.into(),
            attenuation,
            power_level,
        })
    }

    /// Starts the signal generator's CW mode using the expansion module.
    pub fn start_cw_exp(&self, cw: impl Into<Frequency>, power_dbm: f64) -> io::Result<()> {
        self.send_command(super::Command::StartCwExp {
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
        self.send_command(super::Command::StartFreqSweep {
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
        self.send_command(super::Command::StartFreqSweepExp {
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
        self.send_command(super::Command::StartTracking {
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
        self.send_command(super::Command::StartTrackingExp {
            start: start.into(),
            power_dbm,
            sweep_steps,
            step: step.into(),
        })
    }

    /// Jumps to a new frequency using the tracking step frequency.
    pub fn tracking_step(&self, steps: u16) -> io::Result<()> {
        self.send_command(super::Command::TrackingStep(steps))
    }

    /// Sets the callback that is executed when the signal generator receives a `Config`.
    pub fn set_config_callback(&self, cb: impl FnMut(Config) + Send + 'static) {
        *self.message_container().config_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Removes the callback that is executed when the signal generator receives a `Config`.
    pub fn remove_config_callback(&self) {
        *self.message_container().config_callback.lock().unwrap() = None;
    }

    /// Sets the callback that is executed when the signal generator receives a `ConfigExp`.
    pub fn set_config_exp_callback(&self, cb: impl FnMut(ConfigExp) + Send + 'static) {
        *self.message_container().config_exp_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Removes the callback that is executed when the signal generator receives a `ConfigExp`.
    pub fn remove_config_exp_callback(&self) {
        *self.message_container().config_exp_callback.lock().unwrap() = None;
    }

    /// Sets the callback that is executed when the signal generator receives a `ConfigAmpSweep`.
    pub fn set_config_amp_sweep_callback(&self, cb: impl FnMut(ConfigAmpSweep) + Send + 'static) {
        *self
            .message_container()
            .config_amp_sweep_callback
            .lock()
            .unwrap() = Some(Box::new(cb));
    }

    /// Removes the callback that is executed when the signal generator receives a `ConfigAmpSweep`.
    pub fn remove_config_amp_sweep_callback(&self) {
        *self
            .message_container()
            .config_amp_sweep_callback
            .lock()
            .unwrap() = None;
    }

    /// Sets the callback that is executed when the signal generator receives a `ConfigAmpSweepExp`.
    pub fn set_config_amp_sweep_exp_callback(
        &self,
        cb: impl FnMut(ConfigAmpSweepExp) + Send + 'static,
    ) {
        *self
            .message_container()
            .config_amp_sweep_exp_callback
            .lock()
            .unwrap() = Some(Box::new(cb));
    }

    /// Removes the callback that is executed when the signal generator receives a `ConfigAmpSweepExp`.
    pub fn remove_config_amp_sweep_exp_callback(&self) {
        *self
            .message_container()
            .config_amp_sweep_exp_callback
            .lock()
            .unwrap() = None;
    }

    /// Sets the callback that is executed when the signal generator receives a `ConfigCw`.
    pub fn set_config_cw_callback(&self, cb: impl FnMut(ConfigCw) + Send + 'static) {
        *self.message_container().config_cw_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Removes the callback that is executed when the signal generator receives a `ConfigCw`.
    pub fn remove_config_cw_callback(&self) {
        *self.message_container().config_cw_callback.lock().unwrap() = None;
    }

    /// Sets the callback that is executed when the signal generator receives a `ConfigCwExp`.
    pub fn set_config_cw_exp_callback(&self, cb: impl FnMut(ConfigCwExp) + Send + 'static) {
        *self
            .message_container()
            .config_cw_exp_callback
            .lock()
            .unwrap() = Some(Box::new(cb));
    }

    /// Removes the callback that is executed when the signal generator receives a `ConfigCwExp`.
    pub fn remove_config_cw_exp_callback(&self) {
        *self
            .message_container()
            .config_cw_exp_callback
            .lock()
            .unwrap() = None;
    }

    /// Sets the callback that is executed when the signal generator receives a `ConfigFreqSweep`.
    pub fn set_config_freq_sweep_callback(&self, cb: impl FnMut(ConfigFreqSweep) + Send + 'static) {
        *self
            .message_container()
            .config_freq_sweep_callback
            .lock()
            .unwrap() = Some(Box::new(cb));
    }

    /// Removes the callback that is executed when the signal generator receives a `ConfigFreqSweep`.
    pub fn remove_config_freq_sweep_callback(&self) {
        *self
            .message_container()
            .config_freq_sweep_callback
            .lock()
            .unwrap() = None;
    }

    /// Sets the callback that is executed when the signal generator receives a `ConfigFreqSweepExp`.
    pub fn set_config_freq_sweep_exp_callback(
        &self,
        cb: impl FnMut(ConfigFreqSweepExp) + Send + 'static,
    ) {
        *self
            .message_container()
            .config_freq_sweep_exp_callback
            .lock()
            .unwrap() = Some(Box::new(cb));
    }

    /// Removes the callback that is executed when the signal generator receives a `ConfigFreqSweepExp`.
    pub fn remove_config_freq_sweep_exp_callback(&self) {
        *self
            .message_container()
            .config_freq_sweep_exp_callback
            .lock()
            .unwrap() = None;
    }

    /// Turns on RF power with the current power and frequency configuration.
    pub fn rf_power_on(&self) -> io::Result<()> {
        self.send_command(super::Command::RfPowerOn)
    }

    /// Turns off RF power.
    pub fn rf_power_off(&self) -> io::Result<()> {
        self.send_command(super::Command::RfPowerOff)
    }
}

impl Deref for SignalGenerator {
    type Target = RfExplorer<MessageContainer>;
    fn deref(&self) -> &Self::Target {
        &self.rfe
    }
}

#[derive(Default)]
pub struct MessageContainer {
    pub(crate) config: (Mutex<Option<Config>>, Condvar),
    pub(crate) config_callback: Mutex<Callback<Config>>,
    pub(crate) config_exp: (Mutex<Option<ConfigExp>>, Condvar),
    pub(crate) config_exp_callback: Mutex<Callback<ConfigExp>>,
    pub(crate) config_amp_sweep: (Mutex<Option<ConfigAmpSweep>>, Condvar),
    pub(crate) config_amp_sweep_callback: Mutex<Callback<ConfigAmpSweep>>,
    pub(crate) config_amp_sweep_exp: (Mutex<Option<ConfigAmpSweepExp>>, Condvar),
    pub(crate) config_amp_sweep_exp_callback: Mutex<Callback<ConfigAmpSweepExp>>,
    pub(crate) config_cw: (Mutex<Option<ConfigCw>>, Condvar),
    pub(crate) config_cw_callback: Mutex<Callback<ConfigCw>>,
    pub(crate) config_cw_exp: (Mutex<Option<ConfigCwExp>>, Condvar),
    pub(crate) config_cw_exp_callback: Mutex<Callback<ConfigCwExp>>,
    pub(crate) config_freq_sweep: (Mutex<Option<ConfigFreqSweep>>, Condvar),
    pub(crate) config_freq_sweep_callback: Mutex<Callback<ConfigFreqSweep>>,
    pub(crate) config_freq_sweep_exp: (Mutex<Option<ConfigFreqSweepExp>>, Condvar),
    pub(crate) config_freq_sweep_exp_callback: Mutex<Callback<ConfigFreqSweepExp>>,
    pub(crate) screen_data: (Mutex<Option<ScreenData>>, Condvar),
    pub(crate) temperature: (Mutex<Option<Temperature>>, Condvar),
    pub(crate) setup_info: (Mutex<Option<SetupInfo<Model>>>, Condvar),
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
                    cb(config);
                }
            }
            Self::Message::ConfigAmpSweep(config) => {
                *self.config_amp_sweep.0.lock().unwrap() = Some(config);
                self.config_amp_sweep.1.notify_one();
                if let Some(ref mut cb) = *self.config_amp_sweep_callback.lock().unwrap() {
                    cb(config);
                }
            }
            Self::Message::ConfigCw(config) => {
                *self.config_cw.0.lock().unwrap() = Some(config);
                self.config_cw.1.notify_one();
                if let Some(ref mut cb) = *self.config_cw_callback.lock().unwrap() {
                    cb(config);
                }
            }
            Self::Message::ConfigFreqSweep(config) => {
                *self.config_freq_sweep.0.lock().unwrap() = Some(config);
                self.config_freq_sweep.1.notify_one();
                if let Some(ref mut cb) = *self.config_freq_sweep_callback.lock().unwrap() {
                    cb(config);
                }
            }
            Self::Message::ConfigExp(config) => {
                *self.config_exp.0.lock().unwrap() = Some(config);
                self.config_exp.1.notify_one();
                if let Some(ref mut cb) = *self.config_exp_callback.lock().unwrap() {
                    cb(config);
                }
            }
            Self::Message::ConfigAmpSweepExp(config) => {
                *self.config_amp_sweep_exp.0.lock().unwrap() = Some(config);
                self.config_amp_sweep.1.notify_one();
                if let Some(ref mut cb) = *self.config_amp_sweep_exp_callback.lock().unwrap() {
                    cb(config);
                }
            }
            Self::Message::ConfigCwExp(config) => {
                *self.config_cw_exp.0.lock().unwrap() = Some(config);
                self.config_cw_exp.1.notify_one();
                if let Some(ref mut cb) = *self.config_cw_exp_callback.lock().unwrap() {
                    cb(config);
                }
            }
            Self::Message::ConfigFreqSweepExp(config) => {
                *self.config_freq_sweep_exp.0.lock().unwrap() = Some(config);
                self.config_freq_sweep_exp.1.notify_one();
                if let Some(ref mut cb) = *self.config_freq_sweep_exp_callback.lock().unwrap() {
                    cb(config);
                }
            }
            Self::Message::ScreenData(screen_data) => {
                *self.screen_data.0.lock().unwrap() = Some(screen_data);
                self.screen_data.1.notify_one();
            }
            Self::Message::SerialNumber(serial_number) => {
                *self.serial_number.0.lock().unwrap() = Some(serial_number);
                self.serial_number.1.notify_one();
            }
            Self::Message::SetupInfo(setup_info) => {
                *self.setup_info.0.lock().unwrap() = Some(setup_info);
                self.setup_info.1.notify_one();
            }
            Self::Message::Temperature(temperature) => {
                *self.temperature.0.lock().unwrap() = Some(temperature);
                self.temperature.1.notify_one();
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
                RfExplorer::<MessageContainer>::RECEIVE_INITIAL_DEVICE_INFO_TIMEOUT,
                |config| config.is_none(),
            )
            .unwrap()
            .0
            .is_some()
            && setup_info_cvar
                .wait_timeout_while(
                    setup_info_lock.lock().unwrap(),
                    RfExplorer::<MessageContainer>::RECEIVE_INITIAL_DEVICE_INFO_TIMEOUT,
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

impl RfExplorerMessageContainer for MessageContainer {
    type Model = super::Model;

    fn setup_info(&self) -> Option<SetupInfo<Self::Model>> {
        self.setup_info.0.lock().unwrap().clone()
    }

    fn screen_data(&self) -> Option<ScreenData> {
        self.screen_data.0.lock().unwrap().clone()
    }
}

impl Debug for MessageContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageContainer")
            .field("config", &self.config.0.lock().unwrap())
            .field("config_exp", &self.config_exp.0.lock().unwrap())
            .field("config_cw", &self.config_cw.0.lock().unwrap())
            .field("config_cw_exp", &self.config_cw_exp.0.lock().unwrap())
            .field("config_amp_sweep", &self.config_amp_sweep.0.lock().unwrap())
            .field(
                "config_amp_sweep_exp",
                &self.config_amp_sweep_exp.0.lock().unwrap(),
            )
            .field(
                "config_freq_sweep",
                &self.config_freq_sweep.0.lock().unwrap(),
            )
            .field(
                "config_freq_sweep_exp",
                &self.config_freq_sweep_exp.0.lock().unwrap(),
            )
            .field("screen_data", &self.screen_data.0.lock().unwrap())
            .field("temperature", &self.temperature.0.lock().unwrap())
            .field("setup_info", &self.setup_info.0.lock().unwrap())
            .field("serial_number", &self.serial_number.0.lock().unwrap())
            .finish()
    }
}
