use std::slice;

use crate::common::Result;

#[repr(C)]
pub struct Sweep {
    amplitudes_dbm: *mut f32,
    len: usize,
    timestamp: i64,
}

impl From<rfe::spectrum_analyzer::Sweep> for Sweep {
    fn from(sweep: rfe::spectrum_analyzer::Sweep) -> Self {
        let mut amplitudes = sweep.amplitudes_dbm().to_vec().into_boxed_slice();
        let sweep = Sweep {
            amplitudes_dbm: amplitudes.as_mut_ptr(),
            len: amplitudes.len(),
            timestamp: sweep.timestamp().timestamp_millis(),
        };
        std::mem::forget(amplitudes);
        sweep
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_sweep_free(sweep: Sweep) -> Result {
    let amplitudes_dbm = slice::from_raw_parts_mut(sweep.amplitudes_dbm, sweep.len);
    drop(Box::from_raw(amplitudes_dbm.as_mut_ptr()));
    Result::Success
}
