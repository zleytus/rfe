mod command;
mod device;
pub(crate) mod parsers;
mod radio_module;
mod screen_data;
mod serial_number;
mod setup_info;

pub(crate) use command::Command;
pub use device::{RfExplorer, RfExplorerMessageContainer};
pub use radio_module::RadioModule;
pub use screen_data::ScreenData;
pub use serial_number::SerialNumber;
pub use setup_info::SetupInfo;

pub(crate) type Callback<T> = Option<Box<dyn FnMut(T) + Send + 'static>>;
