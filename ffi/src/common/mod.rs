mod callback;
mod result;
mod screen_data;

pub(crate) use callback::UserDataWrapper;
pub use result::Result;

use std::ffi::{CString, c_char};

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
#[unsafe(no_mangle)]
pub extern "C" fn rfe_is_driver_installed() -> bool {
    rfe::is_driver_installed()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_port_names(len: Option<&mut usize>) -> *const *mut c_char {
    let mut port_names = rfe::port_names()
        .iter()
        .map(|name| CString::new(name.as_str()).unwrap_or_default().into_raw())
        .collect::<Vec<*mut c_char>>();
    port_names.shrink_to_fit();

    let port_names_ptr = port_names.as_ptr();
    if let Some(len) = len {
        *len = port_names.len();
    }
    std::mem::forget(port_names);

    port_names_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_free_port_names(port_names_ptr: *mut *mut c_char, len: usize) {
    let port_names = unsafe { Vec::from_raw_parts(port_names_ptr, len, len) };
    for port_name_ptr in port_names {
        let port_name = unsafe { CString::from_raw(port_name_ptr) };
        drop(port_name);
    }
}
