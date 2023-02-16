use std::{
    fmt::Debug,
    io::{self, BufRead},
    sync::{Arc, Condvar, Mutex},
    thread::JoinHandle,
};

use serialport::SerialPortInfo;
use tracing::{debug, warn};

use super::{Config, DspMode, InputStage, Sweep, TrackingStatus};
use crate::common::{
    self, Callback, ConnectionError, ConnectionResult, Device, ScreenData, SerialNumber,
    SerialPortReader, SetupInfo,
};

pub struct SpectrumAnalyzer {
    serial_port: Arc<Mutex<SerialPortReader>>,
    is_reading: Arc<Mutex<bool>>,
    read_thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    pub(crate) config: Arc<(Mutex<Option<Config>>, Condvar)>,
    pub(crate) config_callback: Arc<Mutex<Callback<Config>>>,
    pub(crate) sweep: Arc<(Mutex<Option<Sweep>>, Condvar)>,
    pub(crate) sweep_callback: Arc<Mutex<Callback<Sweep>>>,
    pub(crate) screen_data: Arc<(Mutex<Option<ScreenData>>, Condvar)>,
    pub(crate) dsp_mode: Arc<(Mutex<Option<DspMode>>, Condvar)>,
    pub(crate) tracking_status: Arc<(Mutex<Option<TrackingStatus>>, Condvar)>,
    pub(crate) input_stage: Arc<(Mutex<Option<InputStage>>, Condvar)>,
    pub(crate) setup_info: Arc<(Mutex<Option<SetupInfo>>, Condvar)>,
    serial_number: Arc<(Mutex<Option<SerialNumber>>, Condvar)>,
    port_name: String,
}

impl Device for SpectrumAnalyzer {
    type Message = super::Message;

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
            screen_data: Arc::new((Mutex::new(None), Condvar::new())),
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
        let _ = cvar
            .wait_timeout_while(
                lock.lock().unwrap(),
                SpectrumAnalyzer::RECEIVE_FIRST_CONFIG_TIMEOUT,
                |config| config.is_none(),
            )
            .unwrap();

        // The spectrum analyzer is only valid after receiving a SetupInfo and Config message
        if device.setup_info.0.lock().unwrap().is_some()
            && device.config.0.lock().unwrap().is_some()
        {
            Ok(device)
        } else {
            device.stop_read_thread();
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

    fn stop_read_thread(&self) {
        *self.is_reading.lock().unwrap() = false;
        if let Some(read_thread_handle) = self.read_thread_handle.lock().unwrap().take() {
            let _ = read_thread_handle.join();
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