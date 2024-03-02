mod command;
pub(crate) mod parsers;
mod screen_data;
mod serial_number;
mod setup_info;

pub(crate) use command::Command;
pub use screen_data::ScreenData;
pub(crate) use serial_number::SerialNumber;
pub(crate) use setup_info::SetupInfo;

use std::time::Duration;

pub(crate) type Callback<T> = Option<Box<dyn FnMut(T) + Send + 'static>>;
pub(crate) const NEXT_SCREEN_DATA_TIMEOUT: Duration = Duration::from_secs(2);
pub(crate) const COMMAND_RESPONSE_TIMEOUT: Duration = Duration::from_secs(2);
pub(crate) const RECEIVE_INITIAL_DEVICE_INFO_TIMEOUT: Duration = Duration::from_secs(2);

macro_rules! impl_rf_explorer {
    ($rf_explorer:ident, $message_container:ty) => {
        use crate::common::BaudRate;
        use crate::rf_explorer;
        use std::borrow::Cow;

        #[derive(Debug)]
        pub struct $rf_explorer {
            rfe: Device<$message_container>,
        }

        impl $rf_explorer {
            /// Connects to the first available RF Explorer.
            pub fn connect() -> Option<Self> {
                Some(Self {
                    rfe: Device::connect(Cow::from(rf_explorer::Command::RequestConfig))?,
                })
            }

            /// Connects to the first available RF Explorer with the given name while using the given baud rate.
            pub fn connect_with_name_and_baud_rate(
                name: &str,
                baud_rate: u32,
            ) -> ConnectionResult<Self> {
                Ok(Self {
                    rfe: Device::connect_with_name_and_baud_rate(
                        name,
                        baud_rate,
                        Cow::from(rf_explorer::Command::RequestConfig),
                    )?,
                })
            }

            fn messages(&self) -> &$message_container {
                self.rfe.messages()
            }

            /// The name of the serial port through which the RF Explorer is connected.
            pub fn port_name(&self) -> &str {
                self.rfe.port_name()
            }

            /// The baud rate of the serial connection to the RF Explorer.
            pub fn baud_rate(&self) -> io::Result<u32> {
                self.rfe.baud_rate()
            }

            /// Sets the baud rate of the serial connection to the RF Explorer.
            pub fn set_baud_rate(&self, baud_rate: u32) -> crate::Result<()> {
                let baud_rate = BaudRate::try_from(baud_rate)?;
                self.send_command(rf_explorer::Command::SetBaudRate { baud_rate })?;
                self.rfe
                    .serial_port()
                    .set_baud_rate(baud_rate.bps())
                    .map_err(crate::Error::from)
            }

            /// Sends a command to the RF Explorer.
            pub(crate) fn send_command(
                &self,
                command: impl Into<Cow<'static, [u8]>>,
            ) -> io::Result<()> {
                self.rfe.send_command(command)
            }

            /// Sends bytes to the RF Explorer.
            pub fn send_bytes(&self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
                self.rfe.send_bytes(bytes)
            }

            /// Turns the RF Explorer's LCD on.
            pub fn lcd_on(&self) -> io::Result<()> {
                self.rfe.send_command(rf_explorer::Command::EnableLcd)
            }

            /// Turns the RF Explorer's LCD off.
            pub fn lcd_off(&self) -> io::Result<()> {
                self.rfe.send_command(rf_explorer::Command::DisableLcd)
            }

            /// Tells the RF Explorer to start sending `ScreenData`.
            pub fn enable_dump_screen(&self) -> io::Result<()> {
                self.rfe
                    .send_command(rf_explorer::Command::EnableDumpScreen)
            }

            /// Tells the RF Explorer to stop sending `ScreenData`.
            pub fn disable_dump_screen(&self) -> io::Result<()> {
                self.rfe
                    .send_command(rf_explorer::Command::DisableDumpScreen)
            }

            /// Tells the RF Explorer to stop collecting data.
            pub fn hold(&self) -> io::Result<()> {
                self.rfe.send_command(rf_explorer::Command::Hold)
            }

            /// Reboots the RF Explorer.
            pub fn reboot(&self) -> io::Result<()> {
                self.rfe.send_command(rf_explorer::Command::Reboot)
            }

            /// Turns the RF Explorer's power off.
            pub fn power_off(&self) -> io::Result<()> {
                self.rfe.send_command(rf_explorer::Command::PowerOff)
            }
        }
    };
}

pub(crate) use impl_rf_explorer;
