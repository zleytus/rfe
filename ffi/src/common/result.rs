/// Result code returned by fallible `rfe-ffi` functions.
#[repr(C)]
pub enum Result {
    /// The function completed successfully.
    Success = 0,
    /// The connected device reported unsupported or incompatible firmware.
    IncompatibleFirmwareError,
    /// An argument was invalid, such as an out-of-range value or undersized buffer.
    InvalidInputError,
    /// The requested operation is not valid for the current device state.
    InvalidOperationError,
    /// A serial port or operating system I/O error occurred.
    IoError,
    /// The requested data has not been received from the device.
    NoData,
    /// A required pointer argument was `NULL`.
    NullPtrError,
    /// The device did not respond before the operation timed out.
    TimeoutError,
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
            rfe::Error::IncompatibleFirmware(_) => Result::IncompatibleFirmwareError,
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
