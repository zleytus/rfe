use core::slice;
use std::ffi::{c_char, CString};

use rfe::signal_generator::Model;

use crate::common::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SignalGeneratorModel {
    Rfe6Gen = 60,
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

#[no_mangle]
pub unsafe extern "C" fn rfe_signal_generator_model_name(
    model: SignalGeneratorModel,
    name_buf: Option<&mut c_char>,
    len: usize,
) -> Result {
    let Some(name_buf) = name_buf else {
        return Result::NullPtrError;
    };

    let name = CString::new(Model::from(model).to_string()).unwrap_or_default();
    let name = slice::from_raw_parts(name.as_ptr(), name.as_bytes().len());

    if len < name.len() {
        return Result::InvalidInputError;
    }

    let name_buf = slice::from_raw_parts_mut(name_buf, len);
    name_buf[..name.len()].copy_from_slice(name);

    Result::Success
}

#[no_mangle]
pub unsafe extern "C" fn rfe_signal_generator_model_min_freq_hz(
    model: SignalGeneratorModel,
) -> u64 {
    Model::from(model).min_freq().as_hz()
}

#[no_mangle]
pub unsafe extern "C" fn rfe_signal_generator_model_max_freq_hz(
    model: SignalGeneratorModel,
) -> u64 {
    Model::from(model).max_freq().as_hz()
}
