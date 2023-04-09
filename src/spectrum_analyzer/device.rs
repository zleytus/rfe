use std::{
    fmt::Debug,
    io,
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
};

use tracing::{error, info, trace};

use super::{Config, DspMode, InputStage, Sweep, TrackingStatus};
use crate::common::{
    Callback, Command, ConnectionError, ConnectionResult, Device, ScreenData, SerialNumber,
    SerialPort, SetupInfo,
};

pub struct SpectrumAnalyzer {
    serial_port: SerialPort,
    is_reading: Mutex<bool>,
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

    #[tracing::instrument(skip(serial_port), ret, err)]
    fn connect(serial_port: SerialPort) -> ConnectionResult<Arc<Self>> {
        let device = Arc::new(SpectrumAnalyzer {
            serial_port,
            is_reading: Mutex::new(true),
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
        });

        // Read messages from the RF Explorer on a background thread
        let device_clone = device.clone();
        *device.read_thread_handle.lock().unwrap() = Some(thread::spawn(move || {
            SpectrumAnalyzer::read_messages(device_clone)
        }));

        // Request the SetupInfo and Config from the RF Explorer
        device.serial_port.send_command(Command::RequestConfig)?;

        let (lock, cvar) = &device.setup_info;
        if lock.lock().unwrap().is_none() {
            trace!("Waiting to receive SetupInfo from potential RF Explorer");
            let (_, wait_result) = cvar
                .wait_timeout_while(
                    lock.lock().unwrap(),
                    SpectrumAnalyzer::RECEIVE_INITIAL_SETUP_INFO_TIMEOUT,
                    |setup_info| setup_info.is_none(),
                )
                .unwrap();
            if wait_result.timed_out() {
                error!("Did not receive SetupInfo before timeout");
                device.stop_reading_messages();
                return Err(ConnectionError::Io(io::ErrorKind::TimedOut.into()));
            }
        }

        let (lock, cvar) = &device.config;
        if lock.lock().unwrap().is_none() {
            trace!("Waiting to receive Config from potential RF Explorer");
            let (_, wait_result) = cvar
                .wait_timeout_while(
                    lock.lock().unwrap(),
                    SpectrumAnalyzer::RECEIVE_INITIAL_CONFIG_TIMEOUT,
                    |config| config.is_none(),
                )
                .unwrap();
            if wait_result.timed_out() {
                error!("Did not receive Config before timeout");
                device.stop_reading_messages();
                return Err(ConnectionError::Io(io::ErrorKind::TimedOut.into()));
            }
        }

        // The largest sweep we could receive contains 65,535 (2^16) points
        // To be safe, set the maximum message length to 131,072 (2^17)
        device.serial_port().set_max_message_len(131_072);

        info!("Received SetupInfo and Config before timeout");
        Ok(device)
    }

    fn serial_port(&self) -> &SerialPort {
        &self.serial_port
    }

    fn is_reading(&self) -> bool {
        *self.is_reading.lock().unwrap()
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

    fn serial_number(&self) -> io::Result<SerialNumber> {
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

    fn stop_reading_messages(&self) {
        *self.is_reading.lock().unwrap() = false;
        if let Some(read_thread_handle) = self.read_thread_handle.lock().unwrap().take() {
            let _ = read_thread_handle.join();
        }
    }
}

impl Debug for SpectrumAnalyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectrumAnalyzer")
            .field("setup_info", &self.setup_info)
            .field("config", &self.config)
            .field("serial_port", &self.serial_port)
            .field("serial_number", &self.serial_number.0.lock().unwrap())
            .finish()
    }
}
