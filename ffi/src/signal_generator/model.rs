use core::slice;
use std::ffi::{CString, c_char};

use rfe::signal_generator::Model;

use crate::common::Result;

/// Signal generator model reported by the RF Explorer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SignalGeneratorModel {
    /// Main 6 GHz signal generator module.
    Rfe6Gen = 60,
    /// Expansion 6 GHz signal generator module.
    Rfe6GenExpansion = 61,
}

impl From<Model> for SignalGeneratorModel {
    fn from(model: Model) -> Self {
        match model {
            Model::Rfe6Gen => Self::Rfe6Gen,
            Model::Rfe6GenExpansion => Self::Rfe6GenExpansion,
        }
    }
}

impl From<SignalGeneratorModel> for Model {
    fn from(model: SignalGeneratorModel) -> Self {
        match model {
            SignalGeneratorModel::Rfe6Gen => Self::Rfe6Gen,
            SignalGeneratorModel::Rfe6GenExpansion => Self::Rfe6GenExpansion,
        }
    }
}

/// Writes the display name of a signal generator model.
///
/// `name_buf` must point to a writable buffer of at least `len` bytes. The
/// buffer receives a null-terminated C string. Returns
/// `RESULT_INVALID_INPUT_ERROR` if `len` is too small.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_signal_generator_model_name(
    model: SignalGeneratorModel,
    name_buf: Option<&mut c_char>,
    len: usize,
) -> Result {
    let Some(name_buf) = name_buf else {
        return Result::NullPtrError;
    };

    let name = CString::new(Model::from(model).to_string()).unwrap_or_default();
    let name = unsafe { slice::from_raw_parts(name.as_ptr(), name.as_bytes_with_nul().len()) };

    if len < name.len() {
        return Result::InvalidInputError;
    }

    let name_buf = unsafe { slice::from_raw_parts_mut(name_buf, len) };
    name_buf[..name.len()].copy_from_slice(name);

    Result::Success
}

/// Returns the model's minimum supported output frequency in hertz.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_signal_generator_model_min_freq_hz(
    model: SignalGeneratorModel,
) -> u64 {
    Model::from(model).min_freq().as_hz()
}

/// Returns the model's maximum supported output frequency in hertz.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_signal_generator_model_max_freq_hz(
    model: SignalGeneratorModel,
) -> u64 {
    Model::from(model).max_freq().as_hz()
}
