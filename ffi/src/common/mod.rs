mod callback;
mod result;
mod screen_data;

pub(crate) use callback::UserDataWrapper;
pub use result::Result;

use std::ffi::{CString, c_char};

/// Returns whether the platform RF Explorer USB serial driver appears to be installed.
#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
#[unsafe(no_mangle)]
pub extern "C" fn rfe_is_driver_installed() -> bool {
    rfe::is_driver_installed()
}

/// Returns a heap-allocated array of RF Explorer serial port names.
///
/// If `len` is non-NULL, it is set to the number of returned names. The returned
/// array and each string in it are owned by the caller and must be released with
/// `rfe_free_port_names`.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_port_names(len: Option<&mut usize>) -> *mut *mut c_char {
    let mut port_names = rfe::port_names()
        .iter()
        .map(|name| CString::new(name.as_str()).unwrap_or_default().into_raw())
        .collect::<Vec<*mut c_char>>();
    port_names.shrink_to_fit();

    let port_names_ptr = port_names.as_mut_ptr();
    if let Some(len) = len {
        *len = port_names.len();
    }
    std::mem::forget(port_names);

    port_names_ptr
}

/// Frees an array returned by `rfe_port_names`.
///
/// `len` must be the same length returned by `rfe_port_names`. Passing `NULL`
/// is allowed and has no effect.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_free_port_names(port_names_ptr: *mut *mut c_char, len: usize) {
    if port_names_ptr.is_null() {
        return;
    }

    let port_names = unsafe { Vec::from_raw_parts(port_names_ptr, len, len) };
    for port_name_ptr in port_names {
        let port_name = unsafe { CString::from_raw(port_name_ptr) };
        drop(port_name);
    }
}
