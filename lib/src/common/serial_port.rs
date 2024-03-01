use std::{
    borrow::Cow,
    fmt::Debug,
    io::{self, BufRead, BufReader, Read, Take},
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
    time::Duration,
};

use serialport::{
    DataBits, FlowControl, Parity, SerialPortInfo, SerialPortType, StopBits, UsbPortInfo,
};
use thiserror::Error;
use tracing::{debug, error};

pub(crate) const SLOW_BAUD_RATE: u32 = 2_400;
pub(crate) const FAST_BAUD_RATE: u32 = 500_000;

pub(crate) struct SerialPort {
    buf_reader: Mutex<BufReader<Take<Box<dyn serialport::SerialPort>>>>,
    port_info: SerialPortInfo,
    max_message_len: AtomicU64,
}

impl SerialPort {
    #[tracing::instrument(ret, err)]
    pub(crate) fn open(port_info: &SerialPortInfo, baud_rate: u32) -> ConnectionResult<Self> {
        let serial_port = serialport::new(&port_info.port_name, baud_rate)
            .data_bits(DataBits::Eight)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .timeout(Duration::from_secs(1))
            .open()?;

        const INITIAL_LINE_LIMIT: u64 = 128;

        let buf_reader = if cfg!(target_os = "windows") {
            BufReader::with_capacity(1, serial_port.take(INITIAL_LINE_LIMIT))
        } else {
            BufReader::new(serial_port.take(INITIAL_LINE_LIMIT))
        };

        Ok(SerialPort {
            buf_reader: Mutex::new(buf_reader),
            port_info: port_info.clone(),
            max_message_len: AtomicU64::new(INITIAL_LINE_LIMIT),
        })
    }

    #[tracing::instrument(ret, err)]
    pub(crate) fn open_with_name(name: &str, baud_rate: u32) -> ConnectionResult<Self> {
        let port_info = serialport::available_ports()
            .unwrap_or_default()
            .into_iter()
            .find(|port_info| port_info.port_name == name)
            .ok_or_else(|| ConnectionError::UsbSerialDeviceNotFound(name.to_string()))?;
        Self::open(&port_info, baud_rate)
    }

    #[tracing::instrument(skip(self), err)]
    pub(crate) fn read_line(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut buf_reader = self.buf_reader.lock().unwrap();
        buf_reader
            .get_mut()
            .set_limit(self.max_message_len.load(Ordering::Relaxed));
        buf_reader.read_until(b'\n', buf)
    }

    #[tracing::instrument(skip(self), ret, err, fields(bytes_as_string = String::from_utf8_lossy(bytes.as_ref()).as_ref()))]
    pub(crate) fn send_bytes(&self, bytes: impl AsRef<[u8]> + Debug) -> io::Result<()> {
        self.buf_reader
            .lock()
            .unwrap()
            .get_mut()
            .get_mut()
            .write_all(bytes.as_ref())
    }

    #[tracing::instrument(skip(self))]
    pub(crate) fn send_command(
        &self,
        command: impl Into<Cow<'static, [u8]>> + Debug,
    ) -> io::Result<()> {
        self.send_bytes(command.into())
    }

    pub(crate) fn port_info(&self) -> &SerialPortInfo {
        &self.port_info
    }

    #[tracing::instrument(skip(self), err)]
    pub(crate) fn baud_rate(&self) -> io::Result<u32> {
        self.buf_reader
            .lock()
            .unwrap()
            .get_ref()
            .get_ref()
            .baud_rate()
            .map_err(|err| err.into())
    }

    #[tracing::instrument(skip(self), err)]
    pub(crate) fn set_baud_rate(&self, baud_rate: u32) -> io::Result<()> {
        self.buf_reader
            .lock()
            .unwrap()
            .get_mut()
            .get_mut()
            .set_baud_rate(baud_rate)
            .map_err(|err| err.into())
    }

    pub(crate) fn set_max_message_len(&self, line_limit: u64) {
        self.max_message_len.store(line_limit, Ordering::Relaxed);
    }
}

impl Debug for SerialPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SerialPort")
            .field("port_info", &self.port_info)
            .field("max_message_len", &self.max_message_len)
            .finish()
    }
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("RF Explorer device info was not received")]
    DeviceInfoNotReceived,

    #[error(transparent)]
    InitCommandFailedToSend(#[from] io::Error),

    #[error(transparent)]
    SerialPortFailedToOpen(#[from] serialport::Error),

    #[error("A USB serial device with the name '{0}' could not be found")]
    UsbSerialDeviceNotFound(String),
}

pub type ConnectionResult<T> = Result<T, ConnectionError>;

pub(crate) fn silabs_cp210x_ports() -> impl Iterator<Item = SerialPortInfo> {
    serialport::available_ports()
        .unwrap_or_default()
        .into_iter()
        .filter(is_silabs_cp210x)
}

const fn is_silabs_cp210x(port_info: &SerialPortInfo) -> bool {
    const SILABS_VID: u16 = 4_292;
    const CP210X_PID: u16 = 60_000;
    matches!(
        port_info.port_type,
        SerialPortType::UsbPort(UsbPortInfo {
            vid: SILABS_VID,
            pid: CP210X_PID,
            ..
        })
    )
}

/// Returns the names of serial ports with the VID and PID of an RF Explorer.
///
/// # Examples
///
/// ```
/// for port_name in rfe::port_names() {
///     println!("Port name: {port_name}");
/// }
/// ```
pub fn port_names() -> Vec<String> {
    silabs_cp210x_ports()
        .map(|port_info| port_info.port_name)
        .collect()
}

/// Checks if a driver for the RF Explorer is installed.
#[cfg(target_os = "windows")]
#[tracing::instrument(ret)]
pub fn is_driver_installed() -> bool {
    use std::process::{Command, Stdio};
    let Ok(driver_query) = Command::new("driverquery")
        .stdout(Stdio::piped())
        .spawn() else {
            return false;
        };

    let Ok(mut find_silabs_driver) = Command::new("findstr")
        .arg(r#""/c:"Silicon Labs CP210x""#)
        .stdin(Stdio::from(driver_query.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn() else {
            return false;
        };

    let Ok(exit_status) = find_silabs_driver.wait() else {
        return false;
    };

    debug!(
        driver_search_command = r#"driverquery | findstr /c:"Silicon Labs CP210x""#,
        driver_found = exit_status.success()
    );

    exit_status.success()
}

/// Checks if a driver for the RF Explorer is installed.
#[cfg(target_os = "macos")]
#[tracing::instrument(ret)]
pub fn is_driver_installed() -> bool {
    use std::path::Path;

    let apple_dext_path =
        Path::new("/System/Library/DriverExtensions/com.apple.DriverKit-AppleUSBSLCOM.dext");
    debug!(
        apple_dext_path = ?apple_dext_path,
        apple_dext_path.exists = apple_dext_path.exists()
    );

    let silabs_dext_path =
        Path::new("/Applications/CP210xVCPDriver.app/Contents/Library/SystemExtensions/com.silabs.cp210x.dext");
    debug!(
        silabs_dext_path = ?silabs_dext_path,
        silabs_dext_path.exists = silabs_dext_path.exists()
    );

    apple_dext_path.exists() || silabs_dext_path.exists()
}

/// Checks if a driver for the RF Explorer is installed.
#[cfg(target_os = "linux")]
#[tracing::instrument(ret)]
pub fn is_driver_installed() -> bool {
    use std::process::Command;

    let Ok(mut cp210x_modinfo) = Command::new("modinfo").arg("cp210x").spawn() else {
        return false;
    };

    let Ok(exit_status) = cp210x_modinfo.wait() else {
        return false;
    };

    debug!(
        driver_search_command = "modinfo cp210x",
        driver_found = exit_status.success()
    );

    exit_status.success()
}

fn bps_to_code(baud_rate: u32) -> super::Result<u8> {
    match baud_rate {
        1_200 => Ok(b'1'),
        2_400 => Ok(b'2'),
        4_800 => Ok(b'3'),
        9_600 => Ok(b'4'),
        19_200 => Ok(b'5'),
        38_400 => Ok(b'6'),
        57_600 => Ok(b'7'),
        115_200 => Ok(b'8'),
        500_000 => Ok(b'0'),
        _ => Err(super::Error::InvalidInput("Invalid baud rate".to_string())),
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) struct BaudRate {
    bps: u32,
    code: u8,
}

impl BaudRate {
    pub(crate) fn bps(&self) -> u32 {
        self.bps
    }

    pub(crate) fn code(&self) -> u8 {
        self.code
    }
}

impl TryFrom<u32> for BaudRate {
    type Error = super::Error;

    fn try_from(bps: u32) -> Result<Self, Self::Error> {
        Ok(BaudRate {
            bps,
            code: bps_to_code(bps)?,
        })
    }
}

impl Default for BaudRate {
    fn default() -> Self {
        BaudRate {
            bps: 500_000,
            code: b'0',
        }
    }
}
