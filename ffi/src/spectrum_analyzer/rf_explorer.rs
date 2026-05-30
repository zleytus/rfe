use std::{
    ffi::{CStr, CString, c_char, c_void},
    ptr, slice,
    time::Duration,
};

use rfe::{
    Frequency, ScreenData, SpectrumAnalyzer,
    spectrum_analyzer::{
        CalcMode, Config, DspMode, InputStage, Mode, Model, TrackingStatus, WifiBand,
    },
};

use super::{SpectrumAnalyzerConfig, SpectrumAnalyzerModel};
use crate::common::{Result, UserDataWrapper};

/// Connects to the first RF Explorer spectrum analyzer found on a CP210x USB serial port.
///
/// Returns `NULL` if no compatible device can be opened and initialized. The
/// returned pointer is owned by the caller and must be freed with
/// `rfe_spectrum_analyzer_free`.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_connect() -> *mut SpectrumAnalyzer {
    SpectrumAnalyzer::connect()
        .map(|rfe| Box::into_raw(Box::new(rfe)))
        .unwrap_or(ptr::null_mut())
}

/// Connects to a named serial port using the given baud rate.
///
/// `name` must be a valid null-terminated UTF-8 string. Returns `NULL` if the
/// pointer is null, the string is invalid, or the device cannot be opened and
/// initialized. The returned pointer is owned by the caller and must be freed
/// with `rfe_spectrum_analyzer_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_connect_with_name_and_baud_rate(
    name: Option<&c_char>,
    baud_rate: u32,
) -> *mut SpectrumAnalyzer {
    let Some(Ok(name)) = name.map(|name| unsafe { CStr::from_ptr(name).to_str() }) else {
        return ptr::null_mut();
    };

    SpectrumAnalyzer::connect_with_name_and_baud_rate(name, baud_rate)
        .map(|rfe| Box::into_raw(Box::new(rfe)))
        .unwrap_or(ptr::null_mut())
}

/// Frees a spectrum analyzer returned by `rfe_spectrum_analyzer_connect`.
///
/// Passing `NULL` is allowed and has no effect.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_free(rfe: Option<&mut SpectrumAnalyzer>) {
    if let Some(rfe) = rfe {
        drop(unsafe { Box::from_raw(rfe) });
    }
}

/// Sends raw bytes to the spectrum analyzer.
///
/// `bytes` must point to at least `len` bytes. This function is primarily for
/// advanced users that need to send RF Explorer commands not wrapped by this API.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_send_bytes(
    rfe: Option<&SpectrumAnalyzer>,
    bytes: Option<&u8>,
    len: usize,
) -> Result {
    if let (Some(rfe), Some(bytes)) = (rfe, bytes) {
        let bytes = unsafe { slice::from_raw_parts(bytes, len) };
        rfe.send_bytes(bytes).into()
    } else {
        Result::NullPtrError
    }
}

/// Writes the connected serial port name to a caller-provided buffer.
///
/// Use `rfe_spectrum_analyzer_port_name_len` to get the required buffer size,
/// including the terminating null byte.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_port_name(
    rfe: Option<&SpectrumAnalyzer>,
    port_name_buf: Option<&mut c_char>,
    buf_len: usize,
) -> Result {
    let (Some(rfe), Some(port_name_buf)) = (rfe, port_name_buf) else {
        return Result::NullPtrError;
    };

    let name = CString::new(rfe.port_name()).unwrap_or_default();
    let name = unsafe { slice::from_raw_parts(name.as_ptr(), name.as_bytes_with_nul().len()) };

    if buf_len < name.len() {
        return Result::InvalidInputError;
    }

    let name_buf = unsafe { slice::from_raw_parts_mut(port_name_buf, buf_len) };
    name_buf[..name.len()].copy_from_slice(name);

    Result::Success
}

/// Returns the buffer size required for `rfe_spectrum_analyzer_port_name`.
///
/// The returned size includes the terminating null byte. Returns zero if `rfe`
/// is `NULL`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_port_name_len(
    rfe: Option<&SpectrumAnalyzer>,
) -> usize {
    rfe.map(|rfe| rfe.port_name().len() + 1).unwrap_or_default()
}

/// Writes the firmware version to a caller-provided buffer.
///
/// Use `rfe_spectrum_analyzer_firmware_version_len` to get the required buffer
/// size, including the terminating null byte.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_firmware_version(
    rfe: Option<&SpectrumAnalyzer>,
    firmware_version_buf: Option<&mut c_char>,
    buf_len: usize,
) -> Result {
    let (Some(rfe), Some(firmware_version_buf)) = (rfe, firmware_version_buf) else {
        return Result::NullPtrError;
    };

    let firmware_version = CString::new(rfe.firmware_version()).unwrap_or_default();
    let firmware_version = unsafe {
        slice::from_raw_parts(
            firmware_version.as_ptr(),
            firmware_version.as_bytes_with_nul().len(),
        )
    };

    if buf_len < firmware_version.len() {
        return Result::InvalidInputError;
    }

    let firmware_version_buf = unsafe { slice::from_raw_parts_mut(firmware_version_buf, buf_len) };
    firmware_version_buf[..firmware_version.len()].copy_from_slice(firmware_version);

    Result::Success
}

/// Returns the buffer size required for `rfe_spectrum_analyzer_firmware_version`.
///
/// The returned size includes the terminating null byte. Returns zero if `rfe`
/// is `NULL`.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_firmware_version_len(
    rfe: Option<&SpectrumAnalyzer>,
) -> usize {
    rfe.map(|rfe| rfe.firmware_version().len() + 1)
        .unwrap_or_default()
}

/// Writes the device serial number to a caller-provided buffer.
///
/// Use `rfe_spectrum_analyzer_serial_number_len` to get the required buffer
/// size, including the terminating null byte. Returns `RESULT_NO_DATA` if the
/// device does not report a serial number.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_serial_number(
    rfe: Option<&SpectrumAnalyzer>,
    serial_number_buf: Option<&mut c_char>,
    buf_len: usize,
) -> Result {
    let (Some(rfe), Some(serial_number_buf)) = (rfe, serial_number_buf) else {
        return Result::NullPtrError;
    };

    let Some(serial_number) = rfe.serial_number() else {
        return Result::NoData;
    };

    let serial_number = CString::new(serial_number).unwrap_or_default();
    let serial_number = unsafe {
        slice::from_raw_parts(
            serial_number.as_ptr(),
            serial_number.as_bytes_with_nul().len(),
        )
    };

    if buf_len < serial_number.len() {
        return Result::InvalidInputError;
    }

    let serial_number_buf = unsafe { slice::from_raw_parts_mut(serial_number_buf, buf_len) };
    serial_number_buf[..serial_number.len()].copy_from_slice(serial_number);
    Result::Success
}

/// Returns the buffer size required for `rfe_spectrum_analyzer_serial_number`.
///
/// The returned size includes the terminating null byte. Returns zero if `rfe`
/// is `NULL` or no serial number has been received.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_serial_number_len(rfe: Option<&SpectrumAnalyzer>) -> usize {
    rfe.and_then(SpectrumAnalyzer::serial_number)
        .map(|sn| sn.len() + 1)
        .unwrap_or_default()
}

/// Turns the spectrum analyzer LCD on.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_lcd_on(rfe: Option<&SpectrumAnalyzer>) -> Result {
    if let Some(rfe) = rfe {
        rfe.lcd_on().into()
    } else {
        Result::NullPtrError
    }
}

/// Turns the spectrum analyzer LCD off.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_lcd_off(rfe: Option<&SpectrumAnalyzer>) -> Result {
    if let Some(rfe) = rfe {
        rfe.lcd_off().into()
    } else {
        Result::NullPtrError
    }
}

/// Enables screen dump messages from the spectrum analyzer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_enable_dump_screen(
    rfe: Option<&SpectrumAnalyzer>,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.enable_dump_screen().into()
    } else {
        Result::NullPtrError
    }
}

/// Disables screen dump messages from the spectrum analyzer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_disable_dump_screen(
    rfe: Option<&SpectrumAnalyzer>,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.disable_dump_screen().into()
    } else {
        Result::NullPtrError
    }
}

/// Holds the current spectrum analyzer sweep.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_hold(rfe: Option<&SpectrumAnalyzer>) -> Result {
    if let Some(rfe) = rfe {
        rfe.hold().into()
    } else {
        Result::NullPtrError
    }
}

/// Reboots the spectrum analyzer.
///
/// The `rfe` pointer must not be used after a successful reboot unless the
/// device is reconnected.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_reboot(
    rfe: Option<&mut SpectrumAnalyzer>,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.reboot().into()
    } else {
        Result::NullPtrError
    }
}

/// Powers off the spectrum analyzer.
///
/// The `rfe` pointer must not be used after a successful power-off unless the
/// device is reconnected.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_power_off(
    rfe: Option<&mut SpectrumAnalyzer>,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.power_off().into()
    } else {
        Result::NullPtrError
    }
}

/// Returns the current sweep start frequency in hertz.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_start_freq_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::start_freq)
        .unwrap_or_default()
        .as_hz()
}

/// Returns the current sweep step size in hertz.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_step_size_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::step_size)
        .unwrap_or_default()
        .as_hz()
}

/// Returns the current sweep stop frequency in hertz.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_stop_freq_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::stop_freq)
        .unwrap_or_default()
        .as_hz()
}

/// Returns the current sweep center frequency in hertz.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_center_freq_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::center_freq)
        .unwrap_or_default()
        .as_hz()
}

/// Returns the current sweep span in hertz.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_span_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::span).unwrap_or_default().as_hz()
}

/// Returns the active radio module's minimum supported frequency in hertz.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_min_freq_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::min_freq)
        .unwrap_or_default()
        .as_hz()
}

/// Returns the active radio module's maximum supported frequency in hertz.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_max_freq_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::max_freq)
        .unwrap_or_default()
        .as_hz()
}

/// Returns the active radio module's maximum supported span in hertz.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_max_span_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::max_span)
        .unwrap_or_default()
        .as_hz()
}

/// Returns the resolution bandwidth in hertz.
///
/// Returns zero if the device has not reported a resolution bandwidth.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_rbw_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.and_then(SpectrumAnalyzer::rbw)
        .unwrap_or_default()
        .as_hz()
}

/// Returns the bottom displayed amplitude in dBm.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_min_amp_dbm(rfe: Option<&SpectrumAnalyzer>) -> i16 {
    rfe.map(SpectrumAnalyzer::min_amp_dbm).unwrap_or_default()
}

/// Returns the top displayed amplitude in dBm.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_max_amp_dbm(rfe: Option<&SpectrumAnalyzer>) -> i16 {
    rfe.map(SpectrumAnalyzer::max_amp_dbm).unwrap_or_default()
}

/// Returns the amplitude offset in dB.
///
/// Returns zero if the device has not reported an amplitude offset.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_amp_offset_db(rfe: Option<&SpectrumAnalyzer>) -> i8 {
    rfe.and_then(SpectrumAnalyzer::amp_offset_db)
        .unwrap_or_default()
}

/// Returns the number of points in each sweep.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_sweep_len(rfe: Option<&SpectrumAnalyzer>) -> u16 {
    rfe.map(SpectrumAnalyzer::sweep_len).unwrap_or_default()
}

/// Returns the current operating mode.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_mode(rfe: Option<&SpectrumAnalyzer>) -> Mode {
    rfe.map(SpectrumAnalyzer::mode).unwrap_or_default()
}

/// Returns the current calculator mode.
///
/// Returns the enum default if the device has not reported a calculator mode.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_calc_mode(rfe: Option<&SpectrumAnalyzer>) -> CalcMode {
    rfe.and_then(SpectrumAnalyzer::calc_mode)
        .unwrap_or_default()
}

/// Copies the most recent sweep into a caller-provided buffer.
///
/// `sweep_buf` must point to at least `buf_len` `float` values. If `sweep_len`
/// is non-NULL, it is set to the number of values written. Returns
/// `RESULT_INVALID_INPUT_ERROR` if the buffer is too small, or `RESULT_NO_DATA`
/// if no sweep has been received.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_sweep(
    rfe: Option<&SpectrumAnalyzer>,
    sweep_buf: Option<&mut f32>,
    buf_len: usize,
    sweep_len: Option<&mut usize>,
) -> Result {
    let (Some(rfe), Some(sweep_buf)) = (rfe, sweep_buf) else {
        return Result::NullPtrError;
    };

    match rfe.fill_buf_with_sweep(unsafe { std::slice::from_raw_parts_mut(sweep_buf, buf_len) }) {
        Ok(sweep_length) => {
            if let Some(sweep_len) = sweep_len {
                *sweep_len = sweep_length;
            }
            Result::Success
        }
        Err(error) => error.into(),
    }
}

/// Waits for the next sweep and copies it into a caller-provided buffer.
///
/// `sweep_buf` must point to at least `buf_len` `float` values. If `sweep_len`
/// is non-NULL, it is set to the number of values written.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_wait_for_next_sweep(
    rfe: Option<&SpectrumAnalyzer>,
    sweep_buf: Option<&mut f32>,
    buf_len: usize,
    sweep_len: Option<&mut usize>,
) -> Result {
    let (Some(rfe), Some(sweep_buf)) = (rfe, sweep_buf) else {
        return Result::NullPtrError;
    };

    match rfe.wait_for_next_sweep_and_fill_buf(unsafe {
        std::slice::from_raw_parts_mut(sweep_buf, buf_len)
    }) {
        Ok(sweep_length) => {
            if let Some(sweep_len) = sweep_len {
                *sweep_len = sweep_length;
            }
            Result::Success
        }
        Err(error) => error.into(),
    }
}

/// Waits up to `timeout_secs` seconds for the next sweep and copies it into a buffer.
///
/// `sweep_buf` must point to at least `buf_len` `float` values. If `sweep_len`
/// is non-NULL, it is set to the number of values written.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_wait_for_next_sweep_with_timeout(
    rfe: Option<&SpectrumAnalyzer>,
    timeout_secs: u64,
    sweep_buf: Option<&mut f32>,
    buf_len: usize,
    sweep_len: Option<&mut usize>,
) -> Result {
    let (Some(rfe), Some(sweep_buf)) = (rfe, sweep_buf) else {
        return Result::NullPtrError;
    };

    match rfe
        .wait_for_next_sweep_with_timeout_and_fill_buf(Duration::from_secs(timeout_secs), unsafe {
            std::slice::from_raw_parts_mut(sweep_buf, buf_len)
        }) {
        Ok(sweep_length) => {
            if let Some(sweep_len) = sweep_len {
                *sweep_len = sweep_length;
            }
            Result::Success
        }
        Err(error) => error.into(),
    }
}

/// Returns the most recent LCD screen capture.
///
/// On success, `screen_data` receives a heap-allocated `ScreenData` pointer
/// owned by the caller. Free it with `rfe_screen_data_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_screen_data(
    rfe: Option<&SpectrumAnalyzer>,
    screen_data: Option<&mut *const ScreenData>,
) -> Result {
    let (Some(rfe), Some(screen_data)) = (rfe, screen_data) else {
        return Result::NullPtrError;
    };

    if let Some(data) = rfe.screen_data() {
        *screen_data = Box::into_raw(Box::new(data));
        Result::Success
    } else {
        Result::NoData
    }
}

/// Waits for the next LCD screen capture.
///
/// On success, `screen_data` receives a heap-allocated `ScreenData` pointer
/// owned by the caller. Free it with `rfe_screen_data_free`.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_wait_for_next_screen_data(
    rfe: Option<&SpectrumAnalyzer>,
    screen_data: Option<&mut *const ScreenData>,
) -> Result {
    let (Some(rfe), Some(screen_data)) = (rfe, screen_data) else {
        return Result::NullPtrError;
    };

    match rfe.wait_for_next_screen_data() {
        Ok(data) => {
            *screen_data = Box::into_raw(Box::new(data));
            Result::Success
        }
        Err(error) => error.into(),
    }
}

/// Waits up to `timeout_secs` seconds for the next LCD screen capture.
///
/// On success, `screen_data` receives a heap-allocated `ScreenData` pointer
/// owned by the caller. Free it with `rfe_screen_data_free`.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_wait_for_next_screen_data_with_timeout(
    rfe: Option<&SpectrumAnalyzer>,
    timeout_secs: u64,
    screen_data: Option<&mut *const ScreenData>,
) -> Result {
    let (Some(rfe), Some(screen_data)) = (rfe, screen_data) else {
        return Result::NullPtrError;
    };

    match rfe.wait_for_next_screen_data_with_timeout(Duration::from_secs(timeout_secs)) {
        Ok(data) => {
            *screen_data = Box::into_raw(Box::new(data));
            Result::Success
        }
        Err(error) => error.into(),
    }
}

/// Writes the current DSP mode to `dsp_mode`.
///
/// Returns `RESULT_NO_DATA` if the device has not reported a DSP mode.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_dsp_mode(
    rfe: Option<&SpectrumAnalyzer>,
    dsp_mode: Option<&mut DspMode>,
) -> Result {
    let (Some(rfe), Some(dsp_mode)) = (rfe, dsp_mode) else {
        return Result::NullPtrError;
    };

    if let Some(mode) = rfe.dsp_mode() {
        *dsp_mode = mode;
        Result::Success
    } else {
        Result::NoData
    }
}

/// Writes the current tracking status to `tracking_status`.
///
/// Returns `RESULT_NO_DATA` if the device has not reported a tracking status.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_tracking_status(
    rfe: Option<&SpectrumAnalyzer>,
    tracking_status: Option<&mut TrackingStatus>,
) -> Result {
    let (Some(rfe), Some(tracking_status)) = (rfe, tracking_status) else {
        return Result::NullPtrError;
    };

    if let Some(tracking) = rfe.tracking_status() {
        *tracking_status = tracking;
        Result::Success
    } else {
        Result::NoData
    }
}

/// Writes the current input stage to `input_stage`.
///
/// Returns `RESULT_NO_DATA` if the device has not reported an input stage.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_input_stage(
    rfe: Option<&SpectrumAnalyzer>,
    input_stage: Option<&mut InputStage>,
) -> Result {
    let (Some(rfe), Some(input_stage)) = (rfe, input_stage) else {
        return Result::NullPtrError;
    };

    if let Some(stage) = rfe.input_stage() {
        *input_stage = stage;
        Result::Success
    } else {
        Result::NoData
    }
}

/// Returns the main radio module model.
///
/// Returns `SPECTRUM_ANALYZER_MODEL_UNKNOWN` if no model has been reported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_main_radio_model(
    rfe: Option<&SpectrumAnalyzer>,
) -> SpectrumAnalyzerModel {
    rfe.and_then(|rfe| rfe.main_radio_model())
        .map(Model::into)
        .unwrap_or(SpectrumAnalyzerModel::Unknown)
}

/// Returns the expansion radio module model.
///
/// Returns `SPECTRUM_ANALYZER_MODEL_UNKNOWN` if no expansion model has been reported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_expansion_radio_model(
    rfe: Option<&SpectrumAnalyzer>,
) -> SpectrumAnalyzerModel {
    rfe.and_then(|rfe| rfe.expansion_radio_model())
        .map(Model::into)
        .unwrap_or(SpectrumAnalyzerModel::Unknown)
}

/// Returns the currently active radio module model.
///
/// Returns `SPECTRUM_ANALYZER_MODEL_UNKNOWN` if no model has been reported.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_active_radio_model(
    rfe: Option<&SpectrumAnalyzer>,
) -> SpectrumAnalyzerModel {
    rfe.map(|rfe| rfe.active_radio_model().into())
        .unwrap_or(SpectrumAnalyzerModel::Unknown)
}

/// Returns the currently inactive radio module model.
///
/// Returns `SPECTRUM_ANALYZER_MODEL_UNKNOWN` if no inactive model exists.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_inactive_radio_model(
    rfe: Option<&SpectrumAnalyzer>,
) -> SpectrumAnalyzerModel {
    rfe.and_then(|rfe| rfe.inactive_radio_model())
        .map(Model::into)
        .unwrap_or(SpectrumAnalyzerModel::Unknown)
}

/// Starts Wi-Fi analyzer mode for the requested Wi-Fi band.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_start_wifi_analyzer(
    rfe: Option<&SpectrumAnalyzer>,
    wifi_band: WifiBand,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.start_wifi_analyzer(wifi_band).into()
    } else {
        Result::NullPtrError
    }
}

/// Stops Wi-Fi analyzer mode.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_stop_wifi_analyzer(
    rfe: Option<&SpectrumAnalyzer>,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.stop_wifi_analyzer().into()
    } else {
        Result::NullPtrError
    }
}

/// Requests tracking mode and waits for a tracking status response.
///
/// `start_hz` is the tracking start frequency in hertz and `step_hz` is the
/// tracking step frequency in hertz.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_request_tracking(
    rfe: Option<&SpectrumAnalyzer>,
    start_hz: u64,
    step_hz: u64,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.request_tracking(start_hz, step_hz).into()
    } else {
        Result::NullPtrError
    }
}

/// Steps over the tracking step frequency and makes a measurement.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_tracking_step(
    rfe: Option<&SpectrumAnalyzer>,
    step: u16,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.tracking_step(step).into()
    } else {
        Result::NullPtrError
    }
}

/// Sets the sweep start and stop frequencies in hertz.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_start_stop(
    rfe: Option<&SpectrumAnalyzer>,
    start_hz: u64,
    stop_hz: u64,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_start_stop(start_hz, stop_hz).into()
    } else {
        Result::NullPtrError
    }
}

/// Sets the sweep start frequency, stop frequency, and number of sweep points.
///
/// Frequencies are represented in hertz.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_set_start_stop_sweep_len(
    rfe: Option<&SpectrumAnalyzer>,
    start_hz: u64,
    stop_hz: u64,
    sweep_len: u16,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_start_stop_sweep_len(start_hz, stop_hz, sweep_len)
            .into()
    } else {
        Result::NullPtrError
    }
}

/// Sets the sweep center frequency and span in hertz.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_center_span(
    rfe: Option<&SpectrumAnalyzer>,
    center_hz: u64,
    span_hz: u64,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_center_span(center_hz, span_hz).into()
    } else {
        Result::NullPtrError
    }
}

/// Sets the sweep center frequency, span, and number of sweep points.
///
/// Frequencies are represented in hertz.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_center_span_sweep_len(
    rfe: Option<&SpectrumAnalyzer>,
    center_hz: u64,
    span_hz: u64,
    sweep_len: u16,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_center_span_sweep_len(center_hz, span_hz, sweep_len)
            .into()
    } else {
        Result::NullPtrError
    }
}

/// Sets the minimum and maximum amplitudes displayed on the RF Explorer screen.
///
/// Amplitudes are represented in dBm.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_min_max_amps(
    rfe: Option<&SpectrumAnalyzer>,
    min_amp_dbm: i16,
    max_amp_dbm: i16,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_min_max_amps(min_amp_dbm, max_amp_dbm).into()
    } else {
        Result::NullPtrError
    }
}

/// Sets the callback called when a sweep is received.
///
/// The callback may be invoked from a background thread, and multiple callback
/// invocations may overlap. The `sweep` pointer passed to the callback is only
/// valid for the duration of that callback call. `user_data`, if non-NULL, must
/// remain valid until the callback is removed or the analyzer is freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_sweep_callback(
    rfe: Option<&SpectrumAnalyzer>,
    callback: Option<
        extern "C" fn(
            sweep: *const f32,
            sweep_len: usize,
            start_hz: u64,
            stop_hz: u64,
            user_data: *mut c_void,
        ),
    >,
    user_data: *mut c_void,
) {
    let (Some(rfe), Some(callback)) = (rfe, callback) else {
        return;
    };

    // Wrap the pointer to user_data in our own struct that implements Send so it can be
    // sent across threads
    let user_data = UserDataWrapper(user_data);

    // Convert the C function pointer to a Rust closure
    let cb = move |sweep: &[f32], start_freq: Frequency, stop_freq: Frequency| {
        callback(
            sweep.as_ptr(),
            sweep.len(),
            start_freq.as_hz(),
            stop_freq.as_hz(),
            user_data.clone().0,
        );
    };

    rfe.set_sweep_callback(cb);
}

/// Removes the sweep callback.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_remove_sweep_callback(
    rfe: Option<&SpectrumAnalyzer>,
) {
    if let Some(rfe) = rfe {
        rfe.remove_sweep_callback();
    }
}

/// Sets the callback called when a spectrum analyzer configuration is received.
///
/// The callback may be invoked from a background thread, and multiple callback
/// invocations may overlap. `user_data`, if non-NULL, must remain valid until
/// the callback is removed or the analyzer is freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_config_callback(
    rfe: Option<&SpectrumAnalyzer>,
    callback: Option<extern "C" fn(config: SpectrumAnalyzerConfig, user_data: *mut c_void)>,
    user_data: *mut c_void,
) {
    let (Some(rfe), Some(callback)) = (rfe, callback) else {
        return;
    };

    // Wrap the pointer to user_data in our own struct that implements Send so it can be
    // sent across threads
    let user_data = UserDataWrapper(user_data);

    // Convert the C function pointer to a Rust closure
    let cb = move |config: Config| {
        callback(SpectrumAnalyzerConfig::from(config), user_data.clone().0);
    };

    rfe.set_config_callback(cb);
}

/// Removes the configuration callback.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_remove_config_callback(
    rfe: Option<&SpectrumAnalyzer>,
) {
    if let Some(rfe) = rfe {
        rfe.remove_config_callback();
    }
}

/// Sets the number of points in each sweep.
///
/// Only Plus models support changing the sweep length.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_sweep_len(
    rfe: Option<&SpectrumAnalyzer>,
    sweep_len: u16,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_sweep_len(sweep_len).into()
    } else {
        Result::NullPtrError
    }
}

/// Sets the calculator mode.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_calc_mode(
    rfe: Option<&SpectrumAnalyzer>,
    calc_mode: CalcMode,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_calc_mode(calc_mode).into()
    } else {
        Result::NullPtrError
    }
}

/// Activates the main radio module.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_activate_main_radio(
    rfe: Option<&SpectrumAnalyzer>,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.activate_main_radio().into()
    } else {
        Result::NullPtrError
    }
}

/// Activates the expansion radio module.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_activate_expansion_radio(
    rfe: Option<&SpectrumAnalyzer>,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.activate_expansion_radio().into()
    } else {
        Result::NullPtrError
    }
}

/// Sets the spectrum analyzer input stage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_input_stage(
    rfe: Option<&SpectrumAnalyzer>,
    input_stage: InputStage,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_input_stage(input_stage).into()
    } else {
        Result::NullPtrError
    }
}

/// Sets the amplitude offset in dB.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_offset_db(
    rfe: Option<&SpectrumAnalyzer>,
    offset_db: i8,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_offset_db(offset_db).into()
    } else {
        Result::NullPtrError
    }
}

/// Sets the DSP mode.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_dsp_mode(
    rfe: Option<&SpectrumAnalyzer>,
    dsp_mode: DspMode,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_dsp_mode(dsp_mode).into()
    } else {
        Result::NullPtrError
    }
}
