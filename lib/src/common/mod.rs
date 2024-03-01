mod device;
mod error;
mod frequency;
mod message;
mod serial_port;

pub use device::Device;
pub use error::{Error, Result};
pub use frequency::Frequency;
pub use message::{MessageContainer, MessageParseError};
pub use serial_port::{is_driver_installed, port_names, ConnectionError, ConnectionResult};
pub(crate) use serial_port::{BaudRate, SerialPort};
