#[repr(C)]
pub enum Result {
    Success,
    InvalidInputError,
    InvalidOperationError,
    IoError,
    TimeoutError,
    NullPtrError,
    NoData,
}

impl<T> From<rfe::Result<T>> for Result {
    fn from(result: rfe::Result<T>) -> Self {
        match result {
            Ok(_) => Result::Success,
            Err(error) => error.into(),
        }
    }
}

impl From<rfe::Error> for Result {
    fn from(error: rfe::Error) -> Self {
        match error {
            rfe::Error::IncompatibleFirmware(_) => Result::IoError,
            rfe::Error::InvalidInput(_) => Result::InvalidInputError,
            rfe::Error::InvalidOperation(_) => Result::InvalidOperationError,
            rfe::Error::Io(_) => Result::IoError,
            rfe::Error::TimedOut(_) => Result::TimeoutError,
        }
    }
}

impl From<std::io::Result<()>> for Result {
    fn from(result: std::io::Result<()>) -> Self {
        match result {
            Ok(_) => Result::Success,
            _ => Result::IoError,
        }
    }
}
