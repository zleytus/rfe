mod command;
mod config;
mod config_amp_sweep;
mod config_cw;
mod config_freq_sweep;
mod message;
mod parsers;
mod setup_info;
mod temperature;

pub(crate) use command::Command;
pub use config::{Attenuation, Config, PowerLevel, RfPower};
pub use config_amp_sweep::ConfigAmpSweep;
pub use config_cw::ConfigCw;
pub use config_freq_sweep::ConfigFreqSweep;
pub use message::Message;
pub use temperature::Temperature;

use crate::{
    rf_explorer::{
        self, Callback, ConnectionResult, Device, Frequency, ParseFromBytes, SerialNumber,
        SerialPortReader, SetupInfo,
    },
    Model, RfExplorer,
};
use serialport::SerialPortInfo;
use std::{
    fmt::Debug,
    io::{self, BufRead, ErrorKind},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

pub struct SignalGenerator {
    serial_port: Arc<Mutex<SerialPortReader>>,
    is_reading: Arc<Mutex<bool>>,
    read_thread_handle: Option<JoinHandle<()>>,
    config: Arc<Mutex<Config>>,
    config_callback: Arc<Mutex<Callback<Config>>>,
    config_amp_sweep: Arc<Mutex<Option<ConfigAmpSweep>>>,
    config_amp_sweep_callback: Arc<Mutex<Callback<ConfigAmpSweep>>>,
    config_cw: Arc<Mutex<Option<ConfigCw>>>,
    config_cw_callback: Arc<Mutex<Callback<ConfigCw>>>,
    config_freq_sweep: Arc<Mutex<Option<ConfigFreqSweep>>>,
    config_freq_sweep_callback: Arc<Mutex<Callback<ConfigFreqSweep>>>,
    temperature: Arc<Mutex<Option<Temperature>>>,
    setup_info: SetupInfo<Self>,
    serial_number: SerialNumber,
    port_name: String,
}

impl SignalGenerator {
    /// Spawns a new thread to read messages from the signal generator
    fn start_read_thread(&mut self) {
        assert!(self.read_thread_handle.is_none());

        let is_reading = self.is_reading.clone();
        let serial_port = self.serial_port.clone();
        let config = self.config.clone();
        let config_callback = self.config_callback.clone();
        let config_amp_sweep = self.config_amp_sweep.clone();
        let config_amp_sweep_callback = self.config_amp_sweep_callback.clone();
        let config_cw = self.config_cw.clone();
        let config_cw_callback = self.config_cw_callback.clone();
        let config_freq_sweep = self.config_freq_sweep.clone();
        let config_freq_sweep_callback = self.config_freq_sweep_callback.clone();
        let temperature = self.temperature.clone();

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

                // Try to parse a new amplitude sweep mode config from the message we received
                if let Ok((_, new_config)) = ConfigAmpSweep::parse_from_bytes(&message_buf) {
                    let mut config_amp_sweep = config_amp_sweep.lock().unwrap();
                    *config_amp_sweep = Some(new_config);
                    if let Some(cb) = config_amp_sweep_callback.lock().unwrap().as_mut() {
                        cb(config_amp_sweep.unwrap());
                    }
                    message_buf.clear();
                    continue;
                }

                // Try to parse a new CW mode config from the message we received
                if let Ok((_, new_config)) = ConfigCw::parse_from_bytes(&message_buf) {
                    let mut config_cw = config_cw.lock().unwrap();
                    *config_cw = Some(new_config);
                    if let Some(cb) = config_cw_callback.lock().unwrap().as_mut() {
                        cb(config_cw.unwrap());
                    }
                    message_buf.clear();
                    continue;
                }

                // Try to parse a new frequency sweep mode config from the message we received
                if let Ok((_, new_config)) = ConfigFreqSweep::parse_from_bytes(&message_buf) {
                    let mut config_freq_sweep = config_freq_sweep.lock().unwrap();
                    *config_freq_sweep = Some(new_config);
                    if let Some(cb) = config_freq_sweep_callback.lock().unwrap().as_mut() {
                        cb(config_freq_sweep.unwrap());
                    }
                    message_buf.clear();
                    continue;
                }

                // Try to parse a temperature messagefrom the message we received
                if let Ok((_, new_temperature)) = Temperature::parse_from_bytes(&message_buf) {
                    *temperature.lock().unwrap() = Some(new_temperature);
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

impl Device for SignalGenerator {
    type Config = Config;
    type SetupInfo = SetupInfo<SignalGenerator>;

    fn connect(serial_port_info: &SerialPortInfo) -> ConnectionResult<Self> {
        let mut serial_port = rf_explorer::open(serial_port_info)?;

        let (config, setup_info, serial_number) =
            SignalGenerator::read_initial_messages(&mut serial_port)?;

        let mut signal_generator = SignalGenerator {
            serial_port: Arc::new(Mutex::new(serial_port)),
            is_reading: Arc::new(Mutex::new(false)),
            read_thread_handle: None,
            config: Arc::new(Mutex::new(config)),
            config_callback: Arc::new(Mutex::new(None)),
            config_cw: Arc::new(Mutex::new(None)),
            config_cw_callback: Arc::new(Mutex::new(None)),
            config_amp_sweep: Arc::new(Mutex::new(None)),
            config_amp_sweep_callback: Arc::new(Mutex::new(None)),
            config_freq_sweep: Arc::new(Mutex::new(None)),
            config_freq_sweep_callback: Arc::new(Mutex::new(None)),
            temperature: Arc::new(Mutex::new(None)),
            setup_info,
            serial_number,
            port_name: serial_port_info.port_name.clone(),
        };

        signal_generator.start_read_thread();

        // Wait until the read thread has started before returning
        while !*signal_generator.is_reading.lock().unwrap() {}

        Ok(signal_generator)
    }

    fn send_bytes<'a>(&mut self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
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

impl RfExplorer<SignalGenerator> {
    /// Returns the signal generator's configuration.
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

    /// Returns the signal generator's amplitude sweep mode configuration.
    pub fn config_amp_sweep(&self) -> Option<ConfigAmpSweep> {
        *self.device.config_amp_sweep.lock().unwrap()
    }

    /// Returns the signal generator's CW mode configuration.
    pub fn config_cw(&self) -> Option<ConfigCw> {
        *self.device.config_cw.lock().unwrap()
    }

    /// Returns the signal generator's frequency sweep mode configuration.
    pub fn config_freq_sweep(&self) -> Option<ConfigFreqSweep> {
        *self.device.config_freq_sweep.lock().unwrap()
    }

    /// Returns the signal generator's temperature.
    pub fn temperature(&self) -> Option<Temperature> {
        *self.device.temperature.lock().unwrap()
    }

    /// Starts the signal generator's amplitude sweep mode.
    pub fn start_amp_sweep(
        &mut self,
        cw: impl Into<Frequency>,
        start_attenuation: Attenuation,
        start_power_level: PowerLevel,
        stop_attenuation: Attenuation,
        stop_power_level: PowerLevel,
        step_delay: Duration,
    ) -> io::Result<()> {
        self.send_command(Command::StartAmpSweep {
            cw_freq: cw.into(),
            start_attenuation,
            start_power_level,
            stop_attenuation,
            stop_power_level,
            step_delay,
        })
    }

    /// Starts the signal generator's amplitude sweep mode using the expansion module.
    pub fn start_amp_sweep_exp(
        &mut self,
        cw: impl Into<Frequency>,
        start_power_dbm: f64,
        step_power_db: f64,
        stop_power_dbm: f64,
        step_delay: Duration,
    ) -> io::Result<()> {
        self.send_command(Command::StartAmpSweepExp {
            cw_freq: cw.into(),
            start_power_dbm,
            step_power_db,
            stop_power_dbm,
            step_delay,
        })
    }

    /// Starts the signal generator's CW mode.
    pub fn start_cw(
        &mut self,
        cw: impl Into<Frequency>,
        attenuation: Attenuation,
        power_level: PowerLevel,
    ) -> io::Result<()> {
        self.send_command(Command::StartCw {
            cw_freq: cw.into(),
            attenuation,
            power_level,
        })
    }

    /// Starts the signal generator's CW mode using the expansion module.
    pub fn start_cw_exp(&mut self, cw: impl Into<Frequency>, power_dbm: f64) -> io::Result<()> {
        self.send_command(Command::StartCwExp {
            cw_freq: cw.into(),
            power_dbm,
        })
    }

    /// Starts the signal generator's frequency sweep mode.
    pub fn start_freq_sweep(
        &mut self,
        start: impl Into<Frequency>,
        attenuation: Attenuation,
        power_level: PowerLevel,
        sweep_steps: u16,
        step_hz: u64,
        step_delay: Duration,
    ) -> io::Result<()> {
        self.send_command(Command::StartFreqSweep {
            start_freq: start.into(),
            attenuation,
            power_level,
            sweep_steps,
            step_freq: Frequency::from_hz(step_hz),
            step_delay,
        })
    }

    /// Starts the signal generator's frequency sweep mode using the expansion module.
    pub fn start_freq_sweep_exp(
        &mut self,
        start: impl Into<Frequency>,
        power_dbm: f64,
        sweep_steps: u16,
        step: impl Into<Frequency>,
        step_delay: Duration,
    ) -> io::Result<()> {
        self.send_command(Command::StartFreqSweepExp {
            start_freq: start.into(),
            power_dbm,
            sweep_steps,
            step_freq: step.into(),
            step_delay,
        })
    }

    /// Starts the signal generator's tracking mode.
    pub fn start_tracking(
        &mut self,
        start: impl Into<Frequency>,
        attenuation: Attenuation,
        power_level: PowerLevel,
        sweep_steps: u16,
        step: impl Into<Frequency>,
    ) -> io::Result<()> {
        self.send_command(Command::StartTracking {
            start_freq: start.into(),
            attenuation,
            power_level,
            sweep_steps,
            step_freq: step.into(),
        })
    }

    /// Starts the signal generator's tracking mode using the expansion module.
    pub fn start_tracking_exp(
        &mut self,
        start: impl Into<Frequency>,
        power_dbm: f64,
        sweep_steps: u16,
        step: impl Into<Frequency>,
    ) -> io::Result<()> {
        self.send_command(Command::StartTrackingExp {
            start_freq: start.into(),
            power_dbm,
            sweep_steps,
            step_freq: step.into(),
        })
    }

    /// Jumps to a new frequency using the tracking step frequency.
    pub fn tracking_step(&mut self, steps: u16) -> io::Result<()> {
        self.send_command(Command::TrackingStep(steps))
    }

    /// Sets the callback that is called when the signal generator receives a `Config`.
    pub fn set_config_callback(&mut self, cb: impl FnMut(Config) + Send + 'static) {
        *self.device.config_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Sets the callback that is called when the signal generator receives a `ConfigAmpSweep`.
    pub fn set_config_amp_sweep_callback(
        &mut self,
        cb: impl FnMut(ConfigAmpSweep) + Send + 'static,
    ) {
        *self.device.config_amp_sweep_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Sets the callback that is called when the signal generator receives a `ConfigCw`.
    pub fn set_config_cw_callback(&mut self, cb: impl FnMut(ConfigCw) + Send + 'static) {
        *self.device.config_cw_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Sets the callback that is called when the signal generator receives a `ConfigFreqSweep`.
    pub fn set_config_freq_sweep_callback(
        &mut self,
        cb: impl FnMut(ConfigFreqSweep) + Send + 'static,
    ) {
        *self.device.config_freq_sweep_callback.lock().unwrap() = Some(Box::new(cb));
    }

    /// Turns on RF power with the current power and frequency configuration.
    pub fn rf_power_on(&mut self) -> io::Result<()> {
        self.send_command(Command::RfPowerOn)
    }

    /// Turns off RF power.
    pub fn rf_power_off(&mut self) -> io::Result<()> {
        self.send_command(Command::RfPowerOff)
    }
}

impl Drop for SignalGenerator {
    fn drop(&mut self) {
        *self.is_reading.lock().unwrap() = false;
        if let Some(read_handle) = self.read_thread_handle.take() {
            let _ = read_handle.join();
        }
    }
}

impl Debug for SignalGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignalGenerator")
            .field("port_name", &self.port_name)
            .field("setup_info", &self.setup_info)
            .field("config", &self.config)
            .field("serial_number", &self.serial_number)
            .finish()
    }
}
