use super::Command;
use serialport::{
    DataBits, FlowControl, Parity, SerialPort, SerialPortInfo, SerialPortType, StopBits,
    UsbPortInfo,
};
use std::borrow::Cow;
use std::{
    io::{self, BufReader},
    time::Duration,
};
use thiserror::Error;
use tracing::{error, trace, warn};

const SILICON_LABS_VID: u16 = 4_292;
const CP210X_UART_BRIDGE_PID: u16 = 60_000;
const RF_EXPLORER_BAUD_RATE: u32 = 500_000;

fn is_rf_explorer_serial_port(port_type: &SerialPortType) -> bool {
    matches!(
        port_type,
        SerialPortType::UsbPort(UsbPortInfo {
            vid: SILICON_LABS_VID,
            pid: CP210X_UART_BRIDGE_PID,
            ..
        })
    )
}

#[tracing::instrument]
pub(crate) fn open(port_info: &SerialPortInfo) -> ConnectionResult<SerialPortReader> {
    // On macOS, serial devices show up in /dev twice as /dev/tty.devicename and /dev/cu.devicename
    // For our purposes, we only want to connect to CU (Call-Up) devices
    if cfg!(target_os = "macos") && !port_info.port_name.starts_with("/dev/cu.") {
        return Err(ConnectionError::NotAnRfExplorer);
    }

    if !is_rf_explorer_serial_port(&port_info.port_type) {
        trace!("VID or PID do not match RF Explorer's");
        return Err(ConnectionError::NotAnRfExplorer);
    }

    let mut serial_port = serialport::new(&port_info.port_name, RF_EXPLORER_BAUD_RATE)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_secs(1))
        .open()?;
    trace!("Opened serial port connection to potential RF Explorer");

    serial_port.write_all(&Cow::from(Command::RequestConfig))?;
    trace!("Requested Config and SetupInfo");

    if cfg!(target_os = "windows") {
        Ok(SerialPortReader::with_capacity(1, serial_port))
    } else {
        Ok(SerialPortReader::new(serial_port))
    }
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Attempted to connect to a device that is not an RF Explorer")]
    NotAnRfExplorer,

    #[error(transparent)]
    SerialPort(#[from] serialport::Error),
}

pub type ConnectionResult<T> = Result<T, ConnectionError>;
pub(crate) type SerialPortReader = BufReader<Box<dyn SerialPort>>;

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
        .arg("/c:\"Silicon Labs CP210x\"")
        .stdin(Stdio::from(driver_query.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn() else {
            return false;
        };

    let Ok(exit_status) = find_silabs_driver.wait() else {
        return false;
    };

    debug!(
        driver_search_command = "driverquery | findstr /c:\"Silicon Labs CP210x\"",
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
