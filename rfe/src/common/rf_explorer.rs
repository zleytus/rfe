use std::{borrow::Cow, fmt::Debug, io, time::Duration};

use super::{
    BaudRate, Command, ConnectionResult, Device, MessageContainer, RadioModule, ScreenData,
    SetupInfo,
};

#[derive(Debug)]
pub struct RfExplorer<M: RfExplorerMessageContainer + 'static> {
    device: Device<M>,
}

impl<M: RfExplorerMessageContainer> RfExplorer<M> {
    pub(crate) const NEXT_SCREEN_DATA_TIMEOUT: Duration = Duration::from_secs(2);
    pub(crate) const COMMAND_RESPONSE_TIMEOUT: Duration = Duration::from_secs(2);
    pub(crate) const RECEIVE_INITIAL_DEVICE_INFO_TIMEOUT: Duration = Duration::from_secs(2);

    pub fn connect() -> Option<Self> {
        Some(Self {
            device: Device::connect(Cow::from(Command::RequestConfig))?,
        })
    }

    pub fn connect_with_name_and_baud_rate(name: &str, baud_rate: u32) -> ConnectionResult<Self> {
        Ok(Self {
            device: Device::connect_with_name_and_baud_rate(
                name,
                baud_rate,
                Cow::from(Command::RequestConfig),
            )?,
        })
    }

    pub fn connect_all() -> Vec<Self> {
        Device::connect_all(Cow::from(Command::RequestConfig))
            .into_iter()
            .map(|device| RfExplorer { device })
            .collect()
    }

    pub(crate) fn message_container(&self) -> &M {
        self.device.messages()
    }

    pub fn port_name(&self) -> &str {
        self.device.port_name()
    }

    pub fn baud_rate(&self) -> io::Result<u32> {
        self.device.baud_rate()
    }

    pub fn set_baud_rate(&self, baud_rate: u32) -> crate::Result<()> {
        let baud_rate = BaudRate::try_from(baud_rate)?;
        self.send_command(Command::SetBaudRate { baud_rate })?;
        self.device
            .serial_port()
            .set_baud_rate(baud_rate.bps())
            .map_err(crate::Error::from)
    }

    pub(crate) fn send_command(&self, command: impl Into<Cow<'static, [u8]>>) -> io::Result<()> {
        self.device.send_command(command)
    }

    pub fn send_bytes(&self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
        self.device.send_bytes(bytes)
    }

    pub fn main_radio_module(&self) -> RadioModule<M::Model> {
        if let Some(setup_info) = self.message_container().setup_info() {
            setup_info.main_radio_module
        } else {
            RadioModule::default()
        }
    }

    pub fn expansion_radio_module(&self) -> Option<RadioModule<M::Model>> {
        if let Some(setup_info) = self.message_container().setup_info() {
            setup_info.expansion_radio_module
        } else {
            None
        }
    }

    pub fn firmware_version(&self) -> String {
        if let Some(setup_info) = self.message_container().setup_info() {
            setup_info.firmware_version
        } else {
            String::default()
        }
    }

    pub fn screen_data(&self) -> Option<ScreenData> {
        self.message_container().screen_data()
    }

    pub fn lcd_on(&self) -> io::Result<()> {
        self.device.send_command(super::Command::EnableLcd)
    }

    pub fn lcd_off(&self) -> io::Result<()> {
        self.device.send_command(super::Command::DisableLcd)
    }

    pub fn enable_dump_screen(&self) -> io::Result<()> {
        self.device.send_command(super::Command::EnableDumpScreen)
    }

    pub fn disable_dump_screen(&self) -> io::Result<()> {
        self.device.send_command(super::Command::DisableDumpScreen)
    }

    pub fn hold(&self) -> io::Result<()> {
        self.device.send_command(super::Command::Hold)
    }

    pub fn reboot(&self) -> io::Result<()> {
        self.device.send_command(super::Command::Reboot)
    }

    pub fn power_off(&self) -> io::Result<()> {
        self.device.send_command(super::Command::PowerOff)
    }
}

pub trait RfExplorerMessageContainer: MessageContainer {
    type Model: Debug + Clone + Copy + TryFrom<u8> + PartialEq + Eq + Default;
    fn setup_info(&self) -> Option<SetupInfo<Self::Model>>;
    fn screen_data(&self) -> Option<ScreenData>;
}
