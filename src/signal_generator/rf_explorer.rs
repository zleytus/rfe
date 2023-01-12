use super::{
    Attenuation, Command, Config, ConfigAmpSweep, ConfigCw, ConfigFreqSweep, Message, PowerLevel,
    Temperature,
};
use crate::common::{
    Callback, ConnectionError, ConnectionResult, Device, Error, Result, SerialNumber,
    SerialPortReader, SetupInfo,
};
use crate::{Frequency, Model, RfExplorer, ScreenData};
use serialport::SerialPortInfo;
use std::fmt::Debug;
use std::io::{self, BufRead};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

pub struct SignalGenerator {
    serial_port: Arc<Mutex<SerialPortReader>>,
    is_reading: Arc<Mutex<bool>>,
    read_thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    config: Arc<(Mutex<Option<Config>>, Condvar)>,
    config_callback: Arc<Mutex<Callback<Config>>>,
    config_amp_sweep: Arc<(Mutex<Option<ConfigAmpSweep>>, Condvar)>,
    config_amp_sweep_callback: Arc<Mutex<Callback<ConfigAmpSweep>>>,
    config_cw: Arc<(Mutex<Option<ConfigCw>>, Condvar)>,
    config_cw_callback: Arc<Mutex<Callback<ConfigCw>>>,
    config_freq_sweep: Arc<(Mutex<Option<ConfigFreqSweep>>, Condvar)>,
    config_freq_sweep_callback: Arc<Mutex<Callback<ConfigFreqSweep>>>,
    screen_data: Arc<(Mutex<Option<ScreenData>>, Condvar)>,
    temperature: Arc<(Mutex<Option<Temperature>>, Condvar)>,
    setup_info: Arc<(Mutex<Option<SetupInfo<Self>>>, Condvar)>,
    serial_number: Arc<(Mutex<Option<SerialNumber>>, Condvar)>,
    port_name: String,
}

impl Device for SignalGenerator {
    type Message = crate::signal_generator::Message;

    fn connect(serial_port_info: &SerialPortInfo) -> ConnectionResult<Arc<Self>> {
        let serial_port = crate::common::open(serial_port_info)?;

        let device = Arc::new(SignalGenerator {
            serial_port: Arc::new(Mutex::new(serial_port)),
            is_reading: Arc::new(Mutex::new(true)),
            read_thread_handle: Arc::new(Mutex::new(None)),
            config: Arc::new((Mutex::new(None), Condvar::new())),
            config_callback: Arc::new(Mutex::new(None)),
            config_cw: Arc::new((Mutex::new(None), Condvar::new())),
            config_cw_callback: Arc::new(Mutex::new(None)),
            config_amp_sweep: Arc::new((Mutex::new(None), Condvar::new())),
            config_amp_sweep_callback: Arc::new(Mutex::new(None)),
            config_freq_sweep: Arc::new((Mutex::new(None), Condvar::new())),
            config_freq_sweep_callback: Arc::new(Mutex::new(None)),
            screen_data: Arc::new((Mutex::new(None), Condvar::new())),
            temperature: Arc::new((Mutex::new(None), Condvar::new())),
            setup_info: Arc::new((Mutex::new(None), Condvar::new())),
            serial_number: Arc::new((Mutex::new(None), Condvar::new())),
            port_name: serial_port_info.port_name.clone(),
        });

        *device.read_thread_handle.lock().unwrap() =
            Some(SignalGenerator::spawn_read_thread(device.clone()));

        // Wait to receive a Config before considering this a valid RF Explorer signal generator
        let (lock, cvar) = &*device.config;
        let (_, timeout_result) = cvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SignalGenerator::RECEIVE_FIRST_CONFIG_TIMEOUT,
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
            }
            Message::ConfigAmpSweep(config) => {
                *self.config_amp_sweep.0.lock().unwrap() = Some(config);
                self.config_amp_sweep.1.notify_one();
            }
            Message::ConfigCw(config) => {
                *self.config_cw.0.lock().unwrap() = Some(config);
                self.config_cw.1.notify_one();
            }
            Message::ConfigFreqSweep(config) => {
                *self.config_freq_sweep.0.lock().unwrap() = Some(config);
                self.config_freq_sweep.1.notify_one();
            }
            Message::ScreenData(screen_data) => {
                *self.screen_data.0.lock().unwrap() = Some(screen_data);
                self.screen_data.1.notify_one();
            }
            Message::SerialNumber(serial_number) => {
                *self.serial_number.0.lock().unwrap() = Some(serial_number);
                self.serial_number.1.notify_one();
            }
            Message::SetupInfo(setup_info) => {
                *self.setup_info.0.lock().unwrap() = Some(setup_info);
                self.setup_info.1.notify_one();
            }
            Message::Temperature(temperature) => {
                *self.temperature.0.lock().unwrap() = Some(temperature);
                self.temperature.1.notify_one();
            }
        }
    }

    fn send_bytes(&self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
        self.serial_port
            .lock()
            .unwrap()
            .get_mut()
            .write_all(bytes.as_ref())
    }

    fn port_name(&self) -> &str {
        &self.port_name
    }

    fn firmware_version(&self) -> String {
        if let Some(setup_info) = self.setup_info.0.lock().unwrap().clone() {
            setup_info.firmware_version
        } else {
            String::default()
        }
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

impl Drop for SignalGenerator {
    fn drop(&mut self) {
        *self.is_reading.lock().unwrap() = false;
        if let Some(read_handle) = self.read_thread_handle.lock().unwrap().take() {
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

    /// Returns the `Model` of the RF Explorer's main module.
    pub fn main_module_model(&self) -> Model {
        self.device
            .setup_info
            .0
            .lock()
            .unwrap()
            .clone()
            .expect("RF Explorer should contain SetupInfo")
            .main_module_model
    }

    /// Returns the `Model` of the RF Explorer's expansion module.
    pub fn expansion_module_model(&self) -> Option<Model> {
        self.device
            .setup_info
            .0
            .lock()
            .unwrap()
            .clone()
            .expect("RF Explorer should contain SetupInfo")
            .expansion_module_model
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
