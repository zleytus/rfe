use std::{
    fmt::Debug,
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use tracing::trace;

use super::{
    Config, ConfigAmpSweep, ConfigAmpSweepExp, ConfigCw, ConfigCwExp, ConfigExp, ConfigFreqSweep,
    ConfigFreqSweepExp, Model, Temperature,
};
use crate::common::{Callback, Device, ScreenData, SerialNumber, SerialPort, SetupInfo};

pub struct SignalGenerator {
    serial_port: SerialPort,
    is_reading: AtomicBool,
    read_thread_handle: Mutex<Option<JoinHandle<()>>>,
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
    serial_number: (Mutex<Option<SerialNumber>>, Condvar),
}

const RECEIVE_INITIAL_DEVICE_INFO_TIMEOUT: Duration = Duration::from_secs(2);

impl Device for SignalGenerator {
    type Message = super::Message;

    fn new(serial_port: SerialPort) -> SignalGenerator {
        SignalGenerator {
            serial_port,
            is_reading: AtomicBool::new(true),
            read_thread_handle: Mutex::new(None),
            config: (Mutex::new(None), Condvar::new()),
            config_callback: Mutex::new(None),
            config_exp: (Mutex::new(None), Condvar::new()),
            config_exp_callback: Mutex::new(None),
            config_cw: (Mutex::new(None), Condvar::new()),
            config_cw_callback: Mutex::new(None),
            config_cw_exp: (Mutex::new(None), Condvar::new()),
            config_cw_exp_callback: Mutex::new(None),
            config_amp_sweep: (Mutex::new(None), Condvar::new()),
            config_amp_sweep_callback: Mutex::new(None),
            config_amp_sweep_exp: (Mutex::new(None), Condvar::new()),
            config_amp_sweep_exp_callback: Mutex::new(None),
            config_freq_sweep: (Mutex::new(None), Condvar::new()),
            config_freq_sweep_callback: Mutex::new(None),
            config_freq_sweep_exp: (Mutex::new(None), Condvar::new()),
            config_freq_sweep_exp_callback: Mutex::new(None),
            screen_data: (Mutex::new(None), Condvar::new()),
            temperature: (Mutex::new(None), Condvar::new()),
            setup_info: (Mutex::new(None), Condvar::new()),
            serial_number: (Mutex::new(None), Condvar::new()),
        }
    }

    fn start_read_thread(device: &Arc<Self>) {
        let device_clone = device.clone();
        *device.read_thread_handle.lock().unwrap() = Some(thread::spawn(move || {
            SignalGenerator::read_messages(device_clone)
        }));
    }

    #[tracing::instrument(skip(self))]
    fn request_device_info(&self) -> io::Result<()> {
        self.serial_port
            .send_command(crate::common::Command::RequestConfig)
    }

    #[tracing::instrument(skip(self), ret)]
    fn wait_for_device_info(&self) -> bool {
        let (config_lock, config_cvar) = &self.config;
        let (setup_info_lock, setup_info_cvar) = &self.setup_info;

        // Check to see if we've already received a Config and SetupInfo
        if config_lock.lock().unwrap().is_some() && setup_info_lock.lock().unwrap().is_some() {
            return true;
        }

        // Wait to see if we receive a Config and SetupInfo before timing out
        config_cvar
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
    }

    fn wait_for_serial_number(&self) -> io::Result<SerialNumber> {
        if let Some(ref serial_number) = *self.serial_number.0.lock().unwrap() {
            return Ok(serial_number.clone());
        }

        self.serial_port
            .send_command(crate::common::Command::RequestSerialNumber)?;

        let (lock, cvar) = &self.serial_number;
        trace!("Waiting to receive SerialNumber from RF Explorer");
        let _ = cvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SignalGenerator::RECEIVE_SERIAL_NUMBER_TIMEOUT,
                |serial_number| serial_number.is_none(),
            )
            .unwrap();

        if let Some(ref serial_number) = *self.serial_number.0.lock().unwrap() {
            Ok(serial_number.clone())
        } else {
            Err(io::ErrorKind::TimedOut.into())
        }
    }

    fn serial_port(&self) -> &SerialPort {
        &self.serial_port
    }

    fn is_reading(&self) -> bool {
        self.is_reading.load(Ordering::Relaxed)
    }

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

    fn firmware_version(&self) -> String {
        if let Some(setup_info) = self.setup_info.0.lock().unwrap().as_ref() {
            setup_info.firmware_version.clone()
        } else {
            String::default()
        }
    }

    fn stop_reading_messages(&self) {
        self.is_reading.store(false, Ordering::Relaxed);
        if let Some(read_thread_handle) = self.read_thread_handle.lock().unwrap().take() {
            let _ = read_thread_handle.join();
        }
    }
}

impl Debug for SignalGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignalGenerator")
            .field("serial_port", &self.serial_port)
            .field("setup_info", &self.setup_info.0.lock().unwrap())
            .field("config", &self.config.0.lock().unwrap())
            .field("serial_number", &self.serial_number.0.lock().unwrap())
            .finish()
    }
}
