use std::ptr;

use super::SignalGenerator;

pub(crate) type SignalGeneratorList = Box<[SignalGenerator]>;

#[no_mangle]
pub unsafe extern "C" fn rfe_signal_generator_list_get(
    list: Option<&SignalGeneratorList>,
    index: usize,
) -> *const SignalGenerator {
    let Some(list) = list else {
        return ptr::null();
    };

    if let Some(rfe) = list.get(index) {
        rfe
    } else {
        ptr::null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_signal_generator_list_len(
    list: Option<&SignalGeneratorList>,
) -> usize {
    if let Some(list) = list {
        list.len()
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_signal_generator_list_free(list: Option<&mut SignalGeneratorList>) {
    if let Some(list) = list {
        drop(Box::from_raw(list));
    }
}
