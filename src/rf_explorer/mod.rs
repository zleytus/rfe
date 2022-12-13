mod command;
mod device;
mod error;
mod frequency;
mod message;
mod model;
pub(crate) mod parsers;
mod screen_data;
mod serial_number;
mod serial_port;
mod setup_info;

pub(crate) use command::Command;
pub use device::Device;
pub use error::{Error, Result};
pub use frequency::Frequency;
pub(crate) use message::Message;
pub use model::Model;
pub use screen_data::ScreenData;
pub use serial_number::SerialNumber;
pub(crate) use serial_port::{open, ConnectionError, ConnectionResult, SerialPortReader};
pub use setup_info::SetupInfo;

use crate::SpectrumAnalyzer;
use std::{borrow::Cow, io, time::Instant};

#[derive(Debug)]
pub struct RfExplorer<D: Device = SpectrumAnalyzer> {
    pub(crate) device: D,
}

impl<D: Device> RfExplorer<D> {
    /// Connects to the first available RF Explorer.
    pub fn connect() -> Option<Self> {
        serialport::available_ports()
            .unwrap_or_default()
            .iter()
            .find_map(|port_info| {
                let device = D::connect(port_info).ok()?;
                Some(Self { device })
            })
    }

    /// Connects to an RF Explorer with the provided name.
    pub fn connect_with_name(name: &str) -> Option<Self> {
        let port_info_with_name = serialport::available_ports()
            .unwrap_or_default()
            .into_iter()
            .find(|port_info| port_info.port_name == name)?;

        let device = D::connect(&port_info_with_name).ok()?;
        Some(Self { device })
    }

    /// Connects to all available RF Explorers.
    pub fn connect_all() -> Vec<Self> {
        serialport::available_ports()
            .unwrap_or_default()
            .iter()
            .filter_map(|port_info| {
                let device = D::connect(port_info).ok()?;
                Some(Self { device })
            })
            .collect()
    }

    /// Sends a command to the RF Explorer.
    pub(crate) fn send_command(&self, command: impl Into<Cow<'static, [u8]>>) -> io::Result<()> {
        self.send_bytes(command.into())
    }

    /// Sends bytes to the RF Explorer.
    pub fn send_bytes(&self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
        self.device.send_bytes(bytes)
    }

    pub fn port_name(&self) -> &str {
        self.device.port_name()
    }

    /// Returns the RF Explorer's serial number.
    pub fn serial_number(&self) -> SerialNumber {
        self.device.serial_number()
    }

    /// Turns on the RF Explorer's LCD screen.
    pub fn lcd_on(&self) -> io::Result<()> {
        self.send_command(Command::EnableLcd)
    }

    /// Turns off the RF Explorer's LCD screen.
    pub fn lcd_off(&self) -> io::Result<()> {
        self.send_command(Command::DisableLcd)
    }

    /// Requests the RF Explorer start sending screen data.
    pub fn enable_dump_screen(&self) -> io::Result<()> {
        self.send_command(Command::EnableDumpScreen)
    }

    /// Requests the RF Explorer stop sending screen data.
    pub fn disable_dump_screen(&self) -> io::Result<()> {
        self.send_command(Command::DisableDumpScreen)
    }

    /// Stops the RF Explorer data dump.
    pub fn hold(&self) -> io::Result<()> {
        self.send_command(Command::Hold)
    }

    /// Reboots the RF Explorer.
    pub fn reboot(self) -> io::Result<()> {
        self.send_command(Command::Reboot)
    }

    /// Turns off the RF Explorer.
    pub fn power_off(self) -> io::Result<()> {
        self.send_command(Command::PowerOff)
    }
}

pub(crate) type Callback<T> = Option<Box<dyn FnMut(T) + Send + 'static>>;
