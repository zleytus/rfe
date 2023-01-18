use std::{
    ffi::{c_char, c_void, CStr, CString},
    ptr, slice,
    time::Duration,
};

use rfe::{
    spectrum_analyzer::{
        CalcMode, Config, DspMode, InputStage, RadioModule, TrackingStatus, WifiBand,
    },
    Model, ScreenData,
};

use super::{SpectrumAnalyzer, SpectrumAnalyzerConfig, SpectrumAnalyzerList, Sweep};
use crate::common::{Result, UserDataWrapper};

#[no_mangle]
pub extern "C" fn rfe_spectrum_analyzer_connect() -> *mut SpectrumAnalyzer {
    if let Some(rfe) = rfe::RfExplorer::connect() {
        Box::into_raw(Box::new(rfe))
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_connect_with_name(
    name: Option<&c_char>,
) -> *mut SpectrumAnalyzer {
    let Some(name) = name else {
        return ptr::null_mut();
    };

    let Ok(name) = CStr::from_ptr(name).to_str() else {
        return ptr::null_mut();
    };

    if let Some(rfe) = SpectrumAnalyzer::connect_with_name(name) {
        Box::into_raw(Box::new(rfe))
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn rfe_spectrum_analyzer_connect_all() -> *mut SpectrumAnalyzerList {
    let rfes = SpectrumAnalyzer::connect_all().into_boxed_slice();
    Box::into_raw(Box::new(rfes))
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_free(rfe: Option<&mut SpectrumAnalyzer>) {
    if let Some(rfe) = rfe {
        drop(Box::from_raw(rfe));
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_send_bytes(
    rfe: Option<&SpectrumAnalyzer>,
    bytes: Option<&u8>,
    len: usize,
) -> Result {
    if let (Some(rfe), Some(bytes)) = (rfe, bytes) {
        let bytes = slice::from_raw_parts(bytes, len);
        rfe.send_bytes(bytes).into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_port_name(
    rfe: Option<&SpectrumAnalyzer>,
    port_name_buf: Option<&mut c_char>,
    buf_len: usize,
) -> Result {
    let (Some(rfe), Some(port_name_buf)) = (rfe, port_name_buf) else {
        return Result::NullPtrError;
    };

    let name = CString::new(rfe.port_name()).unwrap_or_default();
    let name = slice::from_raw_parts(name.as_ptr(), name.as_bytes().len());

    if buf_len < name.len() {
        return Result::InvalidInputError;
    }

    let name_buf = slice::from_raw_parts_mut(port_name_buf, buf_len);
    name_buf[..name.len()].copy_from_slice(name);

    Result::Success
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_firmware_version(
    rfe: Option<&SpectrumAnalyzer>,
    firmware_version_buf: Option<&mut c_char>,
    buf_len: usize,
) -> Result {
    let (Some(rfe), Some(firmware_version_buf)) = (rfe, firmware_version_buf) else {
        return Result::NullPtrError;
    };

    let firmware_version = CString::new(rfe.firmware_version()).unwrap_or_default();
    let firmware_version =
        slice::from_raw_parts(firmware_version.as_ptr(), firmware_version.as_bytes().len());

    if buf_len < firmware_version.len() {
        return Result::InvalidInputError;
    }

    let firmware_version_buf = slice::from_raw_parts_mut(firmware_version_buf, buf_len);
    firmware_version_buf[..firmware_version.len()].copy_from_slice(firmware_version);

    Result::Success
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_serial_number(
    rfe: Option<&SpectrumAnalyzer>,
    serial_number_buf: Option<&mut c_char>,
    buf_len: usize,
) -> Result {
    let (Some(rfe), Some(serial_number_buf)) = (rfe, serial_number_buf) else {
        return Result::NullPtrError;
    };

    let serial_number = CString::new(rfe.serial_number().as_str()).unwrap_or_default();
    let serial_number =
        slice::from_raw_parts(serial_number.as_ptr(), serial_number.as_bytes().len());

    if buf_len < serial_number.len() {
        return Result::InvalidInputError;
    }

    let serial_number_buf = slice::from_raw_parts_mut(serial_number_buf, buf_len);
    serial_number_buf[..serial_number.len()].copy_from_slice(serial_number);
    Result::Success
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_lcd_on(rfe: Option<&SpectrumAnalyzer>) -> Result {
    if let Some(rfe) = rfe {
        rfe.lcd_on().into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_lcd_off(rfe: Option<&SpectrumAnalyzer>) -> Result {
    if let Some(rfe) = rfe {
        rfe.lcd_off().into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_enable_dump_screen(
    rfe: Option<&SpectrumAnalyzer>,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.enable_dump_screen().into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_disable_dump_screen(
    rfe: Option<&SpectrumAnalyzer>,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.disable_dump_screen().into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_hold(rfe: Option<&SpectrumAnalyzer>) -> Result {
    if let Some(rfe) = rfe {
        rfe.hold().into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_reboot(
    rfe: Option<&mut SpectrumAnalyzer>,
) -> Result {
    if let Some(rfe) = rfe {
        let rfe = Box::from_raw(rfe);
        rfe.reboot().into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_power_off(
    rfe: Option<&mut SpectrumAnalyzer>,
) -> Result {
    if let Some(rfe) = rfe {
        let rfe = Box::from_raw(rfe);
        rfe.power_off().into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
pub extern "C" fn rfe_spectrum_analyzer_config(
    rfe: Option<&SpectrumAnalyzer>,
    config: Option<&mut SpectrumAnalyzerConfig>,
) -> Result {
    if let (Some(rfe), Some(config)) = (rfe, config) {
        *config = SpectrumAnalyzerConfig::from(rfe.config());
        Result::Success
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
pub extern "C" fn rfe_spectrum_analyzer_sweep(
    rfe: Option<&SpectrumAnalyzer>,
    sweep: Option<&mut Sweep>,
) -> Result {
    let (Some(rfe), Some(sweep)) = (rfe, sweep) else {
        return Result::NullPtrError;
    };

    if let Some(latest_sweep) = rfe.sweep() {
        *sweep = Sweep::from(latest_sweep);
        Result::Success
    } else {
        Result::NoData
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_wait_for_next_sweep(
    rfe: Option<&SpectrumAnalyzer>,
    sweep: Option<&mut Sweep>,
) -> Result {
    let (Some(rfe), Some(sweep)) = (rfe, sweep) else {
        return Result::NullPtrError;
    };

    match rfe.wait_for_next_sweep() {
        Ok(next_sweep) => {
            *sweep = Sweep::from(next_sweep);
            Result::Success
        }
        Err(error) => error.into(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_wait_for_next_sweep_with_timeout(
    rfe: Option<&SpectrumAnalyzer>,
    timeout_secs: u64,
    sweep: Option<&mut Sweep>,
) -> Result {
    let (Some(rfe), Some(sweep)) = (rfe, sweep) else {
        return Result::NullPtrError;
    };

    match rfe.wait_for_next_sweep_with_timeout(Duration::from_secs(timeout_secs)) {
        Ok(next_sweep) => {
            *sweep = Sweep::from(next_sweep);
            Result::Success
        }
        Err(error) => error.into(),
    }
}

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_main_module_model(
    rfe: Option<&SpectrumAnalyzer>,
    model: Option<&mut Model>,
) -> Result {
    if let (Some(rfe), Some(model)) = (rfe, model) {
        *model = rfe.main_module_model();
        Result::Success
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_expansion_module_model(
    rfe: Option<&SpectrumAnalyzer>,
    model: Option<&mut Model>,
) -> Result {
    let (Some(rfe), Some(model)) = (rfe, model) else {
        return Result::NullPtrError;
    };

    if let Some(expansion_model) = rfe.expansion_module_model() {
        *model = expansion_model;
        Result::Success
    } else {
        Result::InvalidOperationError
    }
}

#[no_mangle]
pub extern "C" fn rfe_spectrum_analyzer_active_module(
    rfe: Option<&SpectrumAnalyzer>,
    radio_module: Option<&mut RadioModule>,
) -> Result {
    if let (Some(rfe), Some(radio_module)) = (rfe, radio_module) {
        *radio_module = rfe.active_module();
        Result::Success
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
pub extern "C" fn rfe_spectrum_analyzer_inactive_module(
    rfe: Option<&SpectrumAnalyzer>,
    radio_module: Option<&mut RadioModule>,
) -> Result {
    let (Some(rfe), Some(radio_module)) = (rfe, radio_module) else {
        return Result::NullPtrError;
    };

    if let Some(module) = rfe.inactive_module() {
        *radio_module = module;
        Result::Success
    } else {
        Result::NoData
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_active_module_model(
    rfe: Option<&SpectrumAnalyzer>,
    model: Option<&mut Model>,
) -> Result {
    if let (Some(rfe), Some(model)) = (rfe, model) {
        *model = rfe.active_module_model();
        Result::Success
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_inactive_module_model(
    rfe: Option<&SpectrumAnalyzer>,
    model: Option<&mut Model>,
) -> Result {
    let (Some(rfe), Some(model)) = (rfe, model) else {
        return Result::NullPtrError;
    };

    if let Some(inactive_module_model) = rfe.inactive_module_model() {
        *model = inactive_module_model;
        Result::Success
    } else {
        Result::NoData
    }
}

#[no_mangle]
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

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_stop_wifi_analyzer(
    rfe: Option<&SpectrumAnalyzer>,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.stop_wifi_analyzer().into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
pub extern "C" fn rfe_set_start_stop_sweep_points(
    rfe: Option<&SpectrumAnalyzer>,
    start_hz: u64,
    stop_hz: u64,
    sweep_points: u16,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_start_stop_sweep_points(start_hz, stop_hz, sweep_points)
            .into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
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

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_center_span_sweep_points(
    rfe: Option<&SpectrumAnalyzer>,
    center_hz: u64,
    span_hz: u64,
    sweep_points: u16,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_center_span_sweep_points(center_hz, span_hz, sweep_points)
            .into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
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

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_sweep_callback(
    rfe: Option<&SpectrumAnalyzer>,
    callback: Option<extern "C" fn(sweep: Sweep, user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> Result {
    let Some(rfe) = rfe else {
        return Result::NullPtrError;
    };

    // Wrap the pointer to user_data in our own struct that implements Send so it can be
    // sent across threads
    let user_data = UserDataWrapper(user_data);

    // Convert the C function pointer to a Rust closure
    let cb = move |sweep: rfe::spectrum_analyzer::Sweep| {
        if let Some(cb) = callback {
            cb(Sweep::from(sweep), user_data.clone().0);
        }
    };

    rfe.set_sweep_callback(cb);
    Result::Success
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_config_callback(
    rfe: Option<&SpectrumAnalyzer>,
    callback: Option<extern "C" fn(config: SpectrumAnalyzerConfig, user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> Result {
    let Some(rfe) = rfe else {
        return Result::NullPtrError;
    };

    // Wrap the pointer to user_data in our own struct that implements Send so it can be
    // sent across threads
    let user_data = UserDataWrapper(user_data);

    // Convert the C function pointer to a Rust closure
    let cb = move |config: Config| {
        if let Some(cb) = callback {
            cb(SpectrumAnalyzerConfig::from(config), user_data.clone().0);
        }
    };

    rfe.set_config_callback(cb);
    Result::Success
}

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_sweep_points(
    rfe: Option<&SpectrumAnalyzer>,
    sweep_points: u16,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_sweep_points(sweep_points).into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
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

#[no_mangle]
pub unsafe extern "C" fn rfe_spectrum_analyzer_set_active_radio_module(
    rfe: Option<&SpectrumAnalyzer>,
    radio_module: RadioModule,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.set_active_radio_module(radio_module).into()
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
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