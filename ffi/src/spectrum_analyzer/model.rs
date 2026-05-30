use core::slice;
use std::ffi::{CString, c_char};

use rfe::spectrum_analyzer::Model;

use crate::common::Result;

/// RF Explorer spectrum analyzer model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SpectrumAnalyzerModel {
    /// 433M model.
    Rfe433M = 0,
    /// 868M model.
    Rfe868M = 1,
    /// 915M model.
    Rfe915M = 2,
    /// WSUB1G model.
    RfeWSub1G = 3,
    /// 2.4G model.
    Rfe24G = 4,
    /// WSUB3G model.
    RfeWSub3G = 5,
    /// 6G model.
    Rfe6G = 6,
    /// WSUB1G+ model.
    RfeWSub1GPlus = 10,
    /// Pro Audio model.
    RfeProAudio = 11,
    /// 2.4G+ model.
    Rfe24GPlus = 12,
    /// 4G+ model.
    Rfe4GPlus = 13,
    /// 6G+ model.
    Rfe6GPlus = 14,
    /// MW5G 3 GHz model.
    RfeMW5G3G = 16,
    /// MW5G 4 GHz model.
    RfeMW5G4G = 17,
    /// MW5G 5 GHz model.
    RfeMW5G5G = 18,
    /// Unknown or unsupported model.
    Unknown = 19,
}

impl From<Model> for SpectrumAnalyzerModel {
    fn from(model: Model) -> Self {
        match model {
            Model::Rfe433M => Self::Rfe433M,
            Model::Rfe868M => Self::Rfe868M,
            Model::Rfe915M => Self::Rfe915M,
            Model::RfeWSub1G => Self::RfeWSub1G,
            Model::Rfe24G => Self::Rfe24G,
            Model::RfeWSub3G => Self::RfeWSub3G,
            Model::Rfe6G => Self::Rfe6G,
            Model::RfeWSub1GPlus => Self::RfeWSub1GPlus,
            Model::RfeProAudio => Self::RfeProAudio,
            Model::Rfe24GPlus => Self::Rfe24GPlus,
            Model::Rfe4GPlus => Self::Rfe4GPlus,
            Model::Rfe6GPlus => Self::Rfe6GPlus,
            Model::RfeMW5G3G => Self::RfeMW5G3G,
            Model::RfeMW5G4G => Self::RfeMW5G4G,
            Model::RfeMW5G5G => Self::RfeMW5G5G,
            Model::Unknown => Self::Unknown,
        }
    }
}

impl From<SpectrumAnalyzerModel> for Model {
    fn from(model: SpectrumAnalyzerModel) -> Self {
        match model {
            SpectrumAnalyzerModel::Rfe433M => Self::Rfe433M,
            SpectrumAnalyzerModel::Rfe868M => Self::Rfe868M,
            SpectrumAnalyzerModel::Rfe915M => Self::Rfe915M,
            SpectrumAnalyzerModel::RfeWSub1G => Self::RfeWSub1G,
            SpectrumAnalyzerModel::Rfe24G => Self::Rfe24G,
            SpectrumAnalyzerModel::RfeWSub3G => Self::RfeWSub3G,
            SpectrumAnalyzerModel::Rfe6G => Self::Rfe6G,
            SpectrumAnalyzerModel::RfeWSub1GPlus => Self::RfeWSub1GPlus,
            SpectrumAnalyzerModel::RfeProAudio => Self::RfeProAudio,
            SpectrumAnalyzerModel::Rfe24GPlus => Self::Rfe24GPlus,
            SpectrumAnalyzerModel::Rfe4GPlus => Self::Rfe4GPlus,
            SpectrumAnalyzerModel::Rfe6GPlus => Self::Rfe6GPlus,
            SpectrumAnalyzerModel::RfeMW5G3G => Self::RfeMW5G3G,
            SpectrumAnalyzerModel::RfeMW5G4G => Self::RfeMW5G4G,
            SpectrumAnalyzerModel::RfeMW5G5G => Self::RfeMW5G5G,
            SpectrumAnalyzerModel::Unknown => Self::Unknown,
        }
    }
}

/// Writes the display name of a spectrum analyzer model.
///
/// `name_buf` must point to a writable buffer of at least `len` bytes. The
/// buffer receives a null-terminated C string. Returns
/// `RESULT_INVALID_INPUT_ERROR` if `len` is too small or `model` is invalid.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_model_name(
    model: SpectrumAnalyzerModel,
    name_buf: Option<&mut c_char>,
    len: usize,
) -> Result {
    let Some(name_buf) = name_buf else {
        return Result::NullPtrError;
    };

    let Ok(model) = Model::try_from(model as u8) else {
        return Result::InvalidInputError;
    };
    let name = CString::new(model.to_string()).unwrap_or_default();
    let name = unsafe { slice::from_raw_parts(name.as_ptr(), name.as_bytes_with_nul().len()) };

    if len < name.len() {
        return Result::InvalidInputError;
    }

    let name_buf = unsafe { slice::from_raw_parts_mut(name_buf, len) };
    name_buf[..name.len()].copy_from_slice(name);

    Result::Success
}

/// Returns whether the model supports Plus-model features.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_model_is_plus_model(model: SpectrumAnalyzerModel) -> bool {
    if let Ok(model) = Model::try_from(model as u8) {
        model.is_plus_model()
    } else {
        false
    }
}

/// Returns whether the model supports Wi-Fi analyzer mode.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_model_has_wifi_analyzer(
    model: SpectrumAnalyzerModel,
) -> bool {
    if let Ok(model) = Model::try_from(model as u8) {
        model.has_wifi_analyzer()
    } else {
        false
    }
}

/// Returns the model's minimum supported input frequency in hertz.
///
/// Returns zero if `model` is invalid.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_model_min_freq_hz(model: SpectrumAnalyzerModel) -> u64 {
    if let Ok(model) = Model::try_from(model as u8) {
        model.min_freq().as_hz()
    } else {
        0
    }
}

/// Returns the model's maximum supported input frequency in hertz.
///
/// Returns zero if `model` is invalid.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_model_max_freq_hz(model: SpectrumAnalyzerModel) -> u64 {
    if let Ok(model) = Model::try_from(model as u8) {
        model.max_freq().as_hz()
    } else {
        0
    }
}

/// Returns the model's minimum supported sweep span in hertz.
///
/// Returns zero if `model` is invalid.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_model_min_span_hz(model: SpectrumAnalyzerModel) -> u64 {
    if let Ok(model) = Model::try_from(model as u8) {
        model.min_span().as_hz()
    } else {
        0
    }
}

/// Returns the model's maximum supported sweep span in hertz.
///
/// Returns zero if `model` is invalid.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_model_max_span_hz(model: SpectrumAnalyzerModel) -> u64 {
    if let Ok(model) = Model::try_from(model as u8) {
        model.max_span().as_hz()
    } else {
        0
    }
}
