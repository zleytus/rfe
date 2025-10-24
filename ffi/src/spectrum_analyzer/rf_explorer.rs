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

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_connect() -> *mut SpectrumAnalyzer {
    SpectrumAnalyzer::connect()
        .map(|rfe| Box::into_raw(Box::new(rfe)))
        .unwrap_or(ptr::null_mut())
}

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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_free(rfe: Option<&mut SpectrumAnalyzer>) {
    if let Some(rfe) = rfe {
        drop(unsafe { Box::from_raw(rfe) });
    }
}

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
    let name = unsafe { slice::from_raw_parts(name.as_ptr(), name.as_bytes().len()) };

    if buf_len < name.len() {
        return Result::InvalidInputError;
    }

    let name_buf = unsafe { slice::from_raw_parts_mut(port_name_buf, buf_len) };
    name_buf[..name.len()].copy_from_slice(name);

    Result::Success
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_port_name_len(
    rfe: Option<&SpectrumAnalyzer>,
) -> usize {
    rfe.map(|rfe| rfe.port_name().len()).unwrap_or_default()
}

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
        slice::from_raw_parts(firmware_version.as_ptr(), firmware_version.as_bytes().len())
    };

    if buf_len < firmware_version.len() {
        return Result::InvalidInputError;
    }

    let firmware_version_buf = unsafe { slice::from_raw_parts_mut(firmware_version_buf, buf_len) };
    firmware_version_buf[..firmware_version.len()].copy_from_slice(firmware_version);

    Result::Success
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_firmware_version_len(
    rfe: Option<&SpectrumAnalyzer>,
) -> usize {
    rfe.map(|rfe| rfe.firmware_version().len())
        .unwrap_or_default()
}

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
    let serial_number =
        unsafe { slice::from_raw_parts(serial_number.as_ptr(), serial_number.as_bytes().len()) };

    if buf_len < serial_number.len() {
        return Result::InvalidInputError;
    }

    let serial_number_buf = unsafe { slice::from_raw_parts_mut(serial_number_buf, buf_len) };
    serial_number_buf[..serial_number.len()].copy_from_slice(serial_number);
    Result::Success
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_serial_number_len(rfe: Option<&SpectrumAnalyzer>) -> usize {
    rfe.and_then(SpectrumAnalyzer::serial_number)
        .map(|sn| sn.len())
        .unwrap_or_default()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_lcd_on(rfe: Option<&SpectrumAnalyzer>) -> Result {
    if let Some(rfe) = rfe {
        rfe.lcd_on().into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_lcd_off(rfe: Option<&SpectrumAnalyzer>) -> Result {
    if let Some(rfe) = rfe {
        rfe.lcd_off().into()
    } else {
        Result::NullPtrError
    }
}

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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_hold(rfe: Option<&SpectrumAnalyzer>) -> Result {
    if let Some(rfe) = rfe {
        rfe.hold().into()
    } else {
        Result::NullPtrError
    }
}

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

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_start_freq_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::start_freq)
        .unwrap_or_default()
        .as_hz()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_step_size_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::step_size)
        .unwrap_or_default()
        .as_hz()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_stop_freq_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::stop_freq)
        .unwrap_or_default()
        .as_hz()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_center_freq_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::center_freq)
        .unwrap_or_default()
        .as_hz()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_span_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::span).unwrap_or_default().as_hz()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_min_freq_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::min_freq)
        .unwrap_or_default()
        .as_hz()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_max_freq_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::max_freq)
        .unwrap_or_default()
        .as_hz()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_max_span_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.map(SpectrumAnalyzer::max_span)
        .unwrap_or_default()
        .as_hz()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_rbw_hz(rfe: Option<&SpectrumAnalyzer>) -> u64 {
    rfe.and_then(SpectrumAnalyzer::rbw)
        .unwrap_or_default()
        .as_hz()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_min_amp_dbm(rfe: Option<&SpectrumAnalyzer>) -> i16 {
    rfe.map(SpectrumAnalyzer::min_amp_dbm).unwrap_or_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_max_amp_dbm(rfe: Option<&SpectrumAnalyzer>) -> i16 {
    rfe.map(SpectrumAnalyzer::max_amp_dbm).unwrap_or_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_amp_offset_db(rfe: Option<&SpectrumAnalyzer>) -> i8 {
    rfe.and_then(SpectrumAnalyzer::amp_offset_db)
        .unwrap_or_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_sweep_len(rfe: Option<&SpectrumAnalyzer>) -> u16 {
    rfe.map(SpectrumAnalyzer::sweep_len).unwrap_or_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_mode(rfe: Option<&SpectrumAnalyzer>) -> Mode {
    rfe.map(SpectrumAnalyzer::mode).unwrap_or_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_calc_mode(rfe: Option<&SpectrumAnalyzer>) -> CalcMode {
    rfe.and_then(SpectrumAnalyzer::calc_mode)
        .unwrap_or_default()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_sweep(
    rfe: Option<&SpectrumAnalyzer>,
    sweep_buf: Option<&mut f32>,
    buf_len: usize,
    sweep_len: Option<&mut usize>,
) -> Result {
    let (Some(rfe), Some(sweep_buf), Some(sweep_len)) = (rfe, sweep_buf, sweep_len) else {
        return Result::NullPtrError;
    };

    match rfe.fill_buf_with_sweep(unsafe { std::slice::from_raw_parts_mut(sweep_buf, buf_len) }) {
        Ok(sweep_length) => {
            *sweep_len = sweep_length;
            Result::Success
        }
        Err(error) => error.into(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_wait_for_next_sweep(
    rfe: Option<&SpectrumAnalyzer>,
    sweep_buf: Option<&mut f32>,
    buf_len: usize,
    sweep_len: Option<&mut usize>,
) -> Result {
    let (Some(rfe), Some(sweep_buf), Some(sweep_len)) = (rfe, sweep_buf, sweep_len) else {
        return Result::NullPtrError;
    };

    match rfe.wait_for_next_sweep_and_fill_buf(unsafe {
        std::slice::from_raw_parts_mut(sweep_buf, buf_len)
    }) {
        Ok(sweep_length) => {
            *sweep_len = sweep_length;
            Result::Success
        }
        Err(error) => error.into(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_wait_for_next_sweep_with_timeout(
    rfe: Option<&SpectrumAnalyzer>,
    timeout_secs: u64,
    sweep_buf: Option<&mut f32>,
    buf_len: usize,
    sweep_len: Option<&mut usize>,
) -> Result {
    let (Some(rfe), Some(sweep_buf), Some(sweep_len)) = (rfe, sweep_buf, sweep_len) else {
        return Result::NullPtrError;
    };

    match rfe
        .wait_for_next_sweep_with_timeout_and_fill_buf(Duration::from_secs(timeout_secs), unsafe {
            std::slice::from_raw_parts_mut(sweep_buf, buf_len)
        }) {
        Ok(sweep_length) => {
            *sweep_len = sweep_length;
            Result::Success
        }
        Err(error) => error.into(),
    }
}

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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_main_radio_model(
    rfe: Option<&SpectrumAnalyzer>,
) -> SpectrumAnalyzerModel {
    rfe.and_then(|rfe| rfe.main_radio_model())
        .map(Model::into)
        .unwrap_or(SpectrumAnalyzerModel::Unknown)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_expansion_radio_model(
    rfe: Option<&SpectrumAnalyzer>,
) -> SpectrumAnalyzerModel {
    rfe.and_then(|rfe| rfe.expansion_radio_model())
        .map(Model::into)
        .unwrap_or(SpectrumAnalyzerModel::Unknown)
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_active_radio_model(
    rfe: Option<&SpectrumAnalyzer>,
) -> SpectrumAnalyzerModel {
    rfe.map(|rfe| rfe.active_radio_model().into())
        .unwrap_or(SpectrumAnalyzerModel::Unknown)
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_spectrum_analyzer_inactive_radio_model(
    rfe: Option<&SpectrumAnalyzer>,
) -> SpectrumAnalyzerModel {
    rfe.and_then(|rfe| rfe.inactive_radio_model())
        .map(Model::into)
        .unwrap_or(SpectrumAnalyzerModel::Unknown)
}

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

/// # Safety
///
/// This function is unsafe because:
///
/// ## Callback Function Requirements
/// * The `callback` function pointer must be valid for the entire lifetime of the
///   `SpectrumAnalyzer` instance or until a new callback is registered
/// * The `callback` function must be thread-safe and may be invoked from any thread
/// * Multiple callback invocations may occur concurrently if previous callbacks have
///   not yet completed
///
/// ## User Data Requirements
/// * The `user_data` pointer (if non-NULL) must remain valid for the entire lifetime
///   of the `SpectrumAnalyzer` instance or until a new callback is registered
/// * Multiple callbacks may run concurrently, each receiving the same `user_data` pointer
/// * If your callback **reads** from `user_data`: ensure the data is not being modified
///   by other threads during callback execution
/// * If your callback **writes** to `user_data`: you must provide your own synchronization
///   (e.g., mutexes, atomic operations) to prevent data races between concurrent callbacks
///   or between callbacks and other parts of your program
/// * If `user_data` points to immutable/read-only data: no additional synchronization needed
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_remove_sweep_callback(
    rfe: Option<&SpectrumAnalyzer>,
) {
    if let Some(rfe) = rfe {
        rfe.remove_sweep_callback();
    }
}

/// # Safety
///
/// This function is unsafe because:
///
/// ## Callback Function Requirements
/// * The `callback` function pointer must be valid for the entire lifetime of the
///   `SpectrumAnalyzer` instance or until a new callback is registered
/// * The `callback` function must be thread-safe and may be invoked from any thread
/// * Multiple callback invocations may occur concurrently if previous callbacks have
///   not yet completed
///
/// ## User Data Requirements
/// * The `user_data` pointer (if non-NULL) must remain valid for the entire lifetime
///   of the `SpectrumAnalyzer` instance or until a new callback is registered
/// * Multiple callbacks may run concurrently, each receiving the same `user_data` pointer
/// * If your callback **reads** from `user_data`: ensure the data is not being modified
///   by other threads during callback execution
/// * If your callback **writes** to `user_data`: you must provide your own synchronization
///   (e.g., mutexes, atomic operations) to prevent data races between concurrent callbacks
///   or between callbacks and other parts of your program
/// * If `user_data` points to immutable/read-only data: no additional synchronization needed
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_spectrum_analyzer_remove_config_callback(
    rfe: Option<&SpectrumAnalyzer>,
) {
    if let Some(rfe) = rfe {
        rfe.remove_config_callback();
    }
}

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
