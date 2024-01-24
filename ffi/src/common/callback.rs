use std::ffi::c_void;

#[derive(Clone)]
pub(crate) struct UserDataWrapper(pub(crate) *mut c_void);

unsafe impl Send for UserDataWrapper {}
