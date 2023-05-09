use std::{
    fmt::Debug,
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread::{self, JoinHandle},
};

use tracing::trace;

use super::{Config, DspMode, InputStage, Sweep, TrackingStatus};
use crate::common::{Callback, Device, ScreenData, SerialNumber, SerialPort, SetupInfo};

pub struct SpectrumAnalyzer {
    serial_port: SerialPort,
    is_reading: AtomicBool,
    read_thread_handle: Mutex<Option<JoinHandle<()>>>,
    pub(crate) config: (Mutex<Option<Config>>, Condvar),
    pub(crate) config_callback: Mutex<Callback<Config>>,
    pub(crate) sweep: (Mutex<Option<Sweep>>, Condvar),
    pub(crate) sweep_callback: Mutex<Callback<Sweep>>,
    pub(crate) screen_data: (Mutex<Option<ScreenData>>, Condvar),
    pub(crate) dsp_mode: (Mutex<Option<DspMode>>, Condvar),
    pub(crate) tracking_status: (Mutex<Option<TrackingStatus>>, Condvar),
    pub(crate) input_stage: (Mutex<Option<InputStage>>, Condvar),
    pub(crate) setup_info: (Mutex<Option<SetupInfo>>, Condvar),
    serial_number: (Mutex<Option<SerialNumber>>, Condvar),
}

impl Device for SpectrumAnalyzer {
    type Message = super::Message;
    type Config = Config;
    type SetupInfo = SetupInfo;

    fn new(serial_port: SerialPort) -> SpectrumAnalyzer {
        SpectrumAnalyzer {
            serial_port,
            is_reading: AtomicBool::new(true),
            read_thread_handle: Mutex::new(None),
            config: (Mutex::new(None), Condvar::new()),
            config_callback: Mutex::new(None),
            sweep: (Mutex::new(None), Condvar::new()),
            sweep_callback: Mutex::new(None),
            screen_data: (Mutex::new(None), Condvar::new()),
            dsp_mode: (Mutex::new(None), Condvar::new()),
            tracking_status: (Mutex::new(None), Condvar::new()),
            input_stage: (Mutex::new(None), Condvar::new()),
            setup_info: (Mutex::new(None), Condvar::new()),
            serial_number: (Mutex::new(None), Condvar::new()),
        }
    }

    fn start_read_thread(device: &Arc<Self>) {
        let device_clone = device.clone();
        *device.read_thread_handle.lock().unwrap() = Some(thread::spawn(move || {
            SpectrumAnalyzer::read_messages(device_clone)
        }));
    }

    #[tracing::instrument(skip(self), ret, err)]
    fn wait_for_config(&self) -> io::Result<Config> {
        let (lock, cvar) = &self.config;
        if let Some(config) = *lock.lock().unwrap() {
            return Ok(config);
        }

        if let Some(config) = *cvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::RECEIVE_INITIAL_CONFIG_TIMEOUT,
                |config| config.is_none(),
            )
            .unwrap()
            .0
        {
            Ok(config)
        } else {
            Err(io::ErrorKind::TimedOut.into())
        }
    }

    #[tracing::instrument(skip(self), ret, err)]
    fn wait_for_setup_info(&self) -> io::Result<SetupInfo> {
        let (lock, cvar) = &self.setup_info;
        if let Some(ref setup_info) = *lock.lock().unwrap() {
            return Ok(setup_info.clone());
        }

        if let Some(ref setup_info) = *cvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::RECEIVE_INITIAL_SETUP_INFO_TIMEOUT,
                |setup_info| setup_info.is_none(),
            )
            .unwrap()
            .0
        {
            Ok(setup_info.clone())
        } else {
            Err(io::ErrorKind::TimedOut.into())
        }
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
                SpectrumAnalyzer::RECEIVE_SERIAL_NUMBER_TIMEOUT,
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
            Self::Message::Sweep(sweep) => {
                *self.sweep.0.lock().unwrap() = Some(sweep);
                self.sweep.1.notify_one();
                if let Some(ref mut cb) = *self.sweep_callback.lock().unwrap() {
                    if let Some(ref sweep) = *self.sweep.0.lock().unwrap() {
                        cb(sweep.clone());
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

impl Debug for SpectrumAnalyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectrumAnalyzer")
            .field("serial_port", &self.serial_port)
            .field("setup_info", &self.setup_info.0.lock().unwrap())
            .field("config", &self.config.0.lock().unwrap())
            .field("serial_number", &self.serial_number.0.lock().unwrap())
            .finish()
    }
}
