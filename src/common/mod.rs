mod command;
mod device;
mod error;
mod frequency;
mod message;
pub(crate) mod parsers;
mod radio_module;
mod rf_explorer;
mod screen_data;
mod serial_number;
mod serial_port;
mod setup_info;

pub(crate) use command::Command;
pub(crate) use device::Device;
pub use error::{Error, Result};
pub use frequency::Frequency;
pub use message::{Message, MessageParseError};
pub use radio_module::RadioModule;
pub use rf_explorer::RfExplorer;
pub use screen_data::ScreenData;
pub use serial_number::SerialNumber;
pub(crate) use serial_port::{open, ConnectionError, ConnectionResult, SerialPortReader};
pub(crate) use setup_info::SetupInfo;

pub(crate) type Callback<T> = Option<Box<dyn FnMut(T) + Send + 'static>>;
