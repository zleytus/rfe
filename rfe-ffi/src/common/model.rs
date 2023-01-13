use core::slice;
use std::ffi::{c_char, CString};

use rfe::Model;

use super::Result;

#[no_mangle]
pub unsafe extern "C" fn rfe_model_name(
    model: Model,
    name_buf: Option<&mut c_char>,
    len: usize,
) -> Result {
    let Some(name_buf) = name_buf else {
        return Result::NullPtrError;
    };

    let name = CString::new(model.to_string()).unwrap_or_default();
    let name = slice::from_raw_parts(name.as_ptr(), name.as_bytes().len());

    if len < name.len() {
        return Result::InvalidInputError;
    }

    let name_buf = slice::from_raw_parts_mut(name_buf, len);
    name_buf[..name.len()].copy_from_slice(name);

    Result::Success
}

#[no_mangle]
pub extern "C" fn rfe_model_is_plus(model: Model) -> bool {
    model.is_plus_model()
}

#[no_mangle]
pub extern "C" fn rfe_model_has_wifi_analyzer(model: Model) -> bool {
    model.has_wifi_analyzer()
}

#[no_mangle]
pub extern "C" fn rfe_model_min_freq_hz(model: Model) -> u64 {
    model.min_freq().as_hz()
}

#[no_mangle]
pub extern "C" fn rfe_model_max_freq_hz(model: Model) -> u64 {
    model.max_freq().as_hz()
}

#[no_mangle]
pub extern "C" fn rfe_model_min_span_hz(model: Model) -> u64 {
    model.min_span().as_hz()
}

#[no_mangle]
pub extern "C" fn rfe_model_max_span_hz(model: Model) -> u64 {
    model.max_span().as_hz()
}
