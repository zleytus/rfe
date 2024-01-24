use std::ptr;

use super::SpectrumAnalyzer;

pub(crate) type SpectrumAnalyzerList = Box<[SpectrumAnalyzer]>;

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_list_get(
    list: Option<&SpectrumAnalyzerList>,
    index: usize,
) -> *const SpectrumAnalyzer {
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
pub unsafe extern "C" fn rfe_spectrum_analyzer_list_len(
    list: Option<&SpectrumAnalyzerList>,
) -> usize {
    if let Some(list) = list {
        list.len()
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_list_free(list: Option<&mut SpectrumAnalyzerList>) {
    if let Some(list) = list {
        drop(Box::from_raw(list));
    }
}
