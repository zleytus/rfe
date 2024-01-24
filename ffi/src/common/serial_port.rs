use std::ffi::{c_char, CString};

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
#[no_mangle]
pub extern "C" fn rfe_serial_port_is_driver_installed() -> bool {
    rfe::serial_port::is_driver_installed()
}

#[no_mangle]
pub extern "C" fn rfe_serial_port_port_names(len: Option<&mut usize>) -> *const *mut c_char {
    let mut port_names = rfe::serial_port::port_names()
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

#[no_mangle]
pub unsafe extern "C" fn rfe_serial_port_free_port_names(
    port_names_ptr: *mut *mut c_char,
    len: usize,
) {
    let port_names = Vec::from_raw_parts(port_names_ptr, len, len);
    for port_name_ptr in port_names {
        let port_name = CString::from_raw(port_name_ptr);
        drop(port_name);
    }
}
