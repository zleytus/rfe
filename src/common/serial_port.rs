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

    Ok(SerialPortReader::new(serial_port))
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
