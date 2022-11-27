use rfe::{Model, SignalGenerator, SpectrumAnalyzer};
use std::{
    ffi::{c_char, c_void, CString},
    slice,
};

pub enum RfExplorer {
    SpectrumAnalyzer(Option<rfe::RfExplorer<SpectrumAnalyzer>>),
    SignalGenerator(Option<rfe::RfExplorer<SignalGenerator>>),
}

#[repr(C)]
pub enum RfExplorerResult {
    Success,
    InvalidInputError,
    InvalidOperationError,
    IoError,
    TimeoutError,
    NullPtrError,
}

impl<T> From<rfe::rf_explorer::Result<T>> for RfExplorerResult {
    fn from(result: rfe::rf_explorer::Result<T>) -> Self {
        match result {
            Ok(_) => RfExplorerResult::Success,
            Err(error) => error.into(),
        }
    }
}

impl From<rfe::rf_explorer::Error> for RfExplorerResult {
    fn from(error: rfe::rf_explorer::Error) -> Self {
        match error {
            rfe::rf_explorer::Error::InvalidInput(_) => RfExplorerResult::InvalidInputError,
            rfe::rf_explorer::Error::InvalidOperation(_) => RfExplorerResult::InvalidOperationError,
            rfe::rf_explorer::Error::Io(_) => RfExplorerResult::IoError,
            rfe::rf_explorer::Error::TimedOut(_) => RfExplorerResult::TimeoutError,
        }
    }
}

impl From<std::io::Result<()>> for RfExplorerResult {
    fn from(result: std::io::Result<()>) -> Self {
        match result {
            Ok(_) => RfExplorerResult::Success,
            _ => RfExplorerResult::IoError,
        }
    }
}

#[derive(Clone)]
pub(crate) struct UserDataWrapper(pub(crate) *mut c_void);

unsafe impl Send for UserDataWrapper {}

#[no_mangle]
pub unsafe extern "C" fn rfe_free(rfe: *mut RfExplorer) {
    if rfe.is_null() {
        return;
    }

    drop(Box::from_raw(rfe));
}

#[no_mangle]
pub unsafe extern "C" fn rfe_send_bytes(
    rfe: *mut RfExplorer,
    bytes: *const u8,
    len: usize,
) -> RfExplorerResult {
    if rfe.is_null() || bytes.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    let bytes = slice::from_raw_parts(bytes, len);
    match &mut (*rfe) {
        RfExplorer::SpectrumAnalyzer(Some(rfe)) => rfe.send_bytes(bytes).into(),
        RfExplorer::SignalGenerator(Some(rfe)) => rfe.send_bytes(bytes).into(),
        _ => RfExplorerResult::InvalidOperationError,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_port_name(
    rfe: *const RfExplorer,
    name_buf: *mut c_char,
    buf_len: usize,
) -> RfExplorerResult {
    if rfe.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    let name = CString::new(match &(*rfe) {
        RfExplorer::SpectrumAnalyzer(Some(rfe)) => rfe.port_name(),
        RfExplorer::SignalGenerator(Some(rfe)) => rfe.port_name(),
        _ => return RfExplorerResult::InvalidOperationError,
    })
    .unwrap_or_default();
    let name = slice::from_raw_parts(name.as_ptr(), name.as_bytes().len());

    if buf_len < name.len() {
        return RfExplorerResult::InvalidInputError;
    }

    let name_buf = slice::from_raw_parts_mut(name_buf, buf_len);
    name_buf[..name.len()].copy_from_slice(name);

    RfExplorerResult::Success
}

#[no_mangle]
pub unsafe extern "C" fn rfe_main_module_model(
    rfe: *const RfExplorer,
    model: *mut Model,
) -> RfExplorerResult {
    if rfe.is_null() || model.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    *model = match &(*rfe) {
        RfExplorer::SpectrumAnalyzer(Some(rfe)) => rfe.main_module_model(),
        RfExplorer::SignalGenerator(Some(rfe)) => rfe.main_module_model(),
        _ => return RfExplorerResult::InvalidOperationError,
    };

    RfExplorerResult::Success
}

#[no_mangle]
pub unsafe extern "C" fn rfe_expansion_module_model(
    rfe: *const RfExplorer,
    model: *mut Model,
) -> RfExplorerResult {
    if rfe.is_null() || model.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    *model = match &(*rfe) {
        RfExplorer::SpectrumAnalyzer(Some(rfe)) => rfe.expansion_module_model(),
        RfExplorer::SignalGenerator(Some(rfe)) => rfe.expansion_module_model(),
        _ => return RfExplorerResult::InvalidOperationError,
    };

    RfExplorerResult::Success
}

#[no_mangle]
pub unsafe extern "C" fn rfe_firmware_version(
    rfe: *const RfExplorer,
    firmware_version_buf: *mut c_char,
    buf_len: usize,
) -> RfExplorerResult {
    if rfe.is_null() || firmware_version_buf.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    let firmware_version = CString::new(match &(*rfe) {
        RfExplorer::SpectrumAnalyzer(Some(rfe)) => rfe.firmware_version(),
        RfExplorer::SignalGenerator(Some(rfe)) => rfe.firmware_version(),
        _ => return RfExplorerResult::InvalidOperationError,
    })
    .unwrap_or_default();
    let firmware_version =
        slice::from_raw_parts(firmware_version.as_ptr(), firmware_version.as_bytes().len());

    if buf_len < firmware_version.len() {
        return RfExplorerResult::InvalidInputError;
    }

    let firmware_version_buf = slice::from_raw_parts_mut(firmware_version_buf, buf_len);
    firmware_version_buf[..firmware_version.len()].copy_from_slice(firmware_version);

    RfExplorerResult::Success
}

#[no_mangle]
pub unsafe extern "C" fn rfe_serial_number(
    rfe: *mut RfExplorer,
    serial_number_buf: *mut c_char,
    buf_len: usize,
) -> RfExplorerResult {
    if rfe.is_null() || serial_number_buf.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    let serial_number = match &mut (*rfe) {
        RfExplorer::SpectrumAnalyzer(Some(rfe)) => rfe.serial_number(),
        RfExplorer::SignalGenerator(Some(rfe)) => rfe.serial_number(),
        _ => return RfExplorerResult::InvalidOperationError,
    };

    if let Ok(serial_number) = serial_number {
        let serial_number = CString::new(serial_number.as_str()).unwrap_or_default();
        let serial_number =
            slice::from_raw_parts(serial_number.as_ptr(), serial_number.as_bytes().len());

        if buf_len < serial_number.len() {
            return RfExplorerResult::InvalidInputError;
        }

        let serial_number_buf = slice::from_raw_parts_mut(serial_number_buf, buf_len);
        serial_number_buf[..serial_number.len()].copy_from_slice(serial_number);
        RfExplorerResult::Success
    } else {
        serial_number.into()
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_lcd_on(rfe: *mut RfExplorer) -> RfExplorerResult {
    if rfe.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    match &mut (*rfe) {
        RfExplorer::SpectrumAnalyzer(Some(rfe)) => rfe.lcd_on().into(),
        RfExplorer::SignalGenerator(Some(rfe)) => rfe.lcd_on().into(),
        _ => RfExplorerResult::InvalidOperationError,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_lcd_off(rfe: *mut RfExplorer) -> RfExplorerResult {
    if rfe.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    match &mut (*rfe) {
        RfExplorer::SpectrumAnalyzer(Some(rfe)) => rfe.lcd_off().into(),
        RfExplorer::SignalGenerator(Some(rfe)) => rfe.lcd_off().into(),
        _ => RfExplorerResult::InvalidOperationError,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_enable_dump_screen(rfe: *mut RfExplorer) -> RfExplorerResult {
    if rfe.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    match &mut (*rfe) {
        RfExplorer::SpectrumAnalyzer(Some(rfe)) => rfe.enable_dump_screen().into(),
        RfExplorer::SignalGenerator(Some(rfe)) => rfe.enable_dump_screen().into(),
        _ => RfExplorerResult::InvalidOperationError,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_disable_dump_screen(rfe: *mut RfExplorer) -> RfExplorerResult {
    if rfe.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    match &mut (*rfe) {
        RfExplorer::SpectrumAnalyzer(Some(rfe)) => rfe.disable_dump_screen().into(),
        RfExplorer::SignalGenerator(Some(rfe)) => rfe.disable_dump_screen().into(),
        _ => RfExplorerResult::InvalidOperationError,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_hold(rfe: *mut RfExplorer) -> RfExplorerResult {
    if rfe.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    match &mut (*rfe) {
        RfExplorer::SpectrumAnalyzer(Some(rfe)) => rfe.hold().into(),
        RfExplorer::SignalGenerator(Some(rfe)) => rfe.hold().into(),
        _ => RfExplorerResult::InvalidOperationError,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_reboot(rfe: *mut RfExplorer) -> RfExplorerResult {
    if rfe.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    match &mut (*rfe) {
        RfExplorer::SpectrumAnalyzer(rfe) => {
            if let Some(rfe) = rfe.take() {
                return rfe.reboot().into();
            }
        }
        RfExplorer::SignalGenerator(rfe) => {
            if let Some(rfe) = rfe.take() {
                return rfe.reboot().into();
            }
        }
    };

    RfExplorerResult::InvalidOperationError
}

#[no_mangle]
pub unsafe extern "C" fn rfe_power_off(rfe: *mut RfExplorer) -> RfExplorerResult {
    if rfe.is_null() {
        return RfExplorerResult::NullPtrError;
    }

    match &mut (*rfe) {
        RfExplorer::SpectrumAnalyzer(rfe) => {
            if let Some(rfe) = rfe.take() {
                return rfe.power_off().into();
            }
        }
        RfExplorer::SignalGenerator(rfe) => {
            if let Some(rfe) = rfe.take() {
                return rfe.power_off().into();
            }
        }
    };

    RfExplorerResult::InvalidOperationError
}
