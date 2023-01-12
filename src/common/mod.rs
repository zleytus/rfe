mod command;
mod device;
mod error;
mod frequency;
mod message;
mod model;
pub(crate) mod parsers;
mod rf_explorer;
mod screen_data;
mod serial_number;
mod serial_port;
mod setup_info;

pub(crate) use command::Command;
pub use device::Device;
pub use error::{Error, Result};
pub use frequency::Frequency;
pub use message::{Message, MessageParseError};
pub use model::Model;
pub use rf_explorer::RfExplorer;
pub use screen_data::ScreenData;
pub use serial_number::SerialNumber;
pub(crate) use serial_port::{open, ConnectionError, ConnectionResult, SerialPortReader};
pub use setup_info::SetupInfo;

pub(crate) type Callback<T> = Option<Box<dyn FnMut(T) + Send + 'static>>;
