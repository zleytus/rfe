use std::{
    fmt::Debug,
    io::{self, BufRead},
    sync::{Arc, Condvar, Mutex},
    thread::JoinHandle,
};

use serialport::SerialPortInfo;

use super::{
    Config, ConfigAmpSweep, ConfigAmpSweepExp, ConfigCw, ConfigCwExp, ConfigExp, ConfigFreqSweep,
    ConfigFreqSweepExp, Model, Temperature,
};
use crate::common::{
    Callback, ConnectionError, ConnectionResult, Device, ScreenData, SerialNumber,
    SerialPortReader, SetupInfo,
};

pub struct SignalGenerator {
    serial_port: Arc<Mutex<SerialPortReader>>,
    is_reading: Arc<Mutex<bool>>,
    read_thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    pub(crate) config: Arc<(Mutex<Option<Config>>, Condvar)>,
    pub(crate) config_callback: Arc<Mutex<Callback<Config>>>,
    pub(crate) config_exp: Arc<(Mutex<Option<ConfigExp>>, Condvar)>,
    pub(crate) config_exp_callback: Arc<Mutex<Callback<ConfigExp>>>,
    pub(crate) config_amp_sweep: Arc<(Mutex<Option<ConfigAmpSweep>>, Condvar)>,
    pub(crate) config_amp_sweep_callback: Arc<Mutex<Callback<ConfigAmpSweep>>>,
    pub(crate) config_amp_sweep_exp: Arc<(Mutex<Option<ConfigAmpSweepExp>>, Condvar)>,
    pub(crate) config_amp_sweep_exp_callback: Arc<Mutex<Callback<ConfigAmpSweepExp>>>,
    pub(crate) config_cw: Arc<(Mutex<Option<ConfigCw>>, Condvar)>,
    pub(crate) config_cw_callback: Arc<Mutex<Callback<ConfigCw>>>,
    pub(crate) config_cw_exp: Arc<(Mutex<Option<ConfigCwExp>>, Condvar)>,
    pub(crate) config_cw_exp_callback: Arc<Mutex<Callback<ConfigCwExp>>>,
    pub(crate) config_freq_sweep: Arc<(Mutex<Option<ConfigFreqSweep>>, Condvar)>,
    pub(crate) config_freq_sweep_callback: Arc<Mutex<Callback<ConfigFreqSweep>>>,
    pub(crate) config_freq_sweep_exp: Arc<(Mutex<Option<ConfigFreqSweepExp>>, Condvar)>,
    pub(crate) config_freq_sweep_exp_callback: Arc<Mutex<Callback<ConfigFreqSweepExp>>>,
    pub(crate) screen_data: Arc<(Mutex<Option<ScreenData>>, Condvar)>,
    pub(crate) temperature: Arc<(Mutex<Option<Temperature>>, Condvar)>,
    pub(crate) setup_info: Arc<(Mutex<Option<SetupInfo<Model>>>, Condvar)>,
    serial_number: Arc<(Mutex<Option<SerialNumber>>, Condvar)>,
    port_name: String,
}

impl Device for SignalGenerator {
    type Message = super::Message;

    fn connect(serial_port_info: &SerialPortInfo) -> ConnectionResult<Arc<Self>> {
        let serial_port = crate::common::open(serial_port_info)?;

        let device = Arc::new(SignalGenerator {
            serial_port: Arc::new(Mutex::new(serial_port)),
            is_reading: Arc::new(Mutex::new(true)),
            read_thread_handle: Arc::new(Mutex::new(None)),
            config: Arc::new((Mutex::new(None), Condvar::new())),
            config_callback: Arc::new(Mutex::new(None)),
            config_exp: Arc::new((Mutex::new(None), Condvar::new())),
            config_exp_callback: Arc::new(Mutex::new(None)),
            config_cw: Arc::new((Mutex::new(None), Condvar::new())),
            config_cw_callback: Arc::new(Mutex::new(None)),
            config_cw_exp: Arc::new((Mutex::new(None), Condvar::new())),
            config_cw_exp_callback: Arc::new(Mutex::new(None)),
            config_amp_sweep: Arc::new((Mutex::new(None), Condvar::new())),
            config_amp_sweep_callback: Arc::new(Mutex::new(None)),
            config_amp_sweep_exp: Arc::new((Mutex::new(None), Condvar::new())),
            config_amp_sweep_exp_callback: Arc::new(Mutex::new(None)),
            config_freq_sweep: Arc::new((Mutex::new(None), Condvar::new())),
            config_freq_sweep_callback: Arc::new(Mutex::new(None)),
            config_freq_sweep_exp: Arc::new((Mutex::new(None), Condvar::new())),
            config_freq_sweep_exp_callback: Arc::new(Mutex::new(None)),
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
        let _ = cvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SignalGenerator::RECEIVE_FIRST_CONFIG_TIMEOUT,
                |config| config.is_none(),
            )
            .unwrap();

        // The signal generator is only valid after receiving a SetupInfo and Config message
        if device.setup_info.0.lock().unwrap().is_some()
            && device.config.0.lock().unwrap().is_some()
        {
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

    fn process_message(&self, message: Self::Message) {
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
        if let Some(setup_info) = self.setup_info.0.lock().unwrap().as_ref() {
            setup_info.firmware_version.clone()
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
