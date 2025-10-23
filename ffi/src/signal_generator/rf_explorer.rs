use std::{
    ffi::{CStr, CString, c_char, c_void},
    ptr, slice,
    time::Duration,
};

use rfe::{
    ScreenData,
    signal_generator::{
        Attenuation, Config, ConfigAmpSweep, ConfigCw, ConfigFreqSweep, PowerLevel,
        SignalGenerator, Temperature,
    },
};

use super::{
    SignalGeneratorConfig, SignalGeneratorConfigAmpSweep, SignalGeneratorConfigCw,
    SignalGeneratorConfigFreqSweep, SignalGeneratorModel,
};
use crate::common::{Result, UserDataWrapper};

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_connect() -> *mut SignalGenerator {
    SignalGenerator::connect()
        .map(|rfe| Box::into_raw(Box::new(rfe)))
        .unwrap_or(ptr::null_mut())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_signal_generator_connect_with_name_and_baud_rate(
    name: Option<&c_char>,
    baud_rate: u32,
) -> *mut SignalGenerator {
    let Some(Ok(name)) = name.map(|name| unsafe { CStr::from_ptr(name).to_str() }) else {
        return ptr::null_mut();
    };

    SignalGenerator::connect_with_name_and_baud_rate(name, baud_rate)
        .map(|rfe| Box::into_raw(Box::new(rfe)))
        .unwrap_or(ptr::null_mut())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_signal_generator_free(rfe: Option<&mut SignalGenerator>) {
    if let Some(rfe) = rfe {
        drop(unsafe { Box::from_raw(rfe) });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_signal_generator_send_bytes(
    rfe: Option<&SignalGenerator>,
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
pub unsafe extern "C" fn rfe_signal_generator_port_name(
    rfe: Option<&SignalGenerator>,
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
pub unsafe extern "C" fn rfe_signal_generator_firmware_version(
    rfe: Option<&SignalGenerator>,
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
pub extern "C" fn rfe_signal_generator_firmware_version_len(
    rfe: Option<&SignalGenerator>,
) -> usize {
    rfe.map(|rfe| rfe.firmware_version().len())
        .unwrap_or_default()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_signal_generator_serial_number(
    rfe: Option<&SignalGenerator>,
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
pub extern "C" fn rfe_signal_generator_serial_number_len(rfe: Option<&SignalGenerator>) -> usize {
    rfe.and_then(SignalGenerator::serial_number)
        .map(|sn| sn.len())
        .unwrap_or_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_lcd_on(rfe: Option<&SignalGenerator>) -> Result {
    if let Some(rfe) = rfe {
        rfe.lcd_on().into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_lcd_off(rfe: Option<&SignalGenerator>) -> Result {
    if let Some(rfe) = rfe {
        rfe.lcd_off().into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_enable_dump_screen(rfe: Option<&SignalGenerator>) -> Result {
    if let Some(rfe) = rfe {
        rfe.enable_dump_screen().into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_disable_dump_screen(
    rfe: Option<&SignalGenerator>,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.disable_dump_screen().into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_hold(rfe: Option<&SignalGenerator>) -> Result {
    if let Some(rfe) = rfe {
        rfe.hold().into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_signal_generator_reboot(rfe: Option<&mut SignalGenerator>) -> Result {
    if let Some(rfe) = rfe {
        rfe.reboot().into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_signal_generator_power_off(
    rfe: Option<&mut SignalGenerator>,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.power_off().into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_config(
    rfe: Option<&SignalGenerator>,
    config: Option<&mut SignalGeneratorConfig>,
) -> Result {
    let (Some(rfe), Some(config)) = (rfe, config) else {
        return Result::NullPtrError;
    };

    if let Some(c) = rfe.config() {
        *config = SignalGeneratorConfig::from(c);
        Result::Success
    } else {
        Result::NoData
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_config_amp_sweep(
    rfe: Option<&SignalGenerator>,
    config: Option<&mut SignalGeneratorConfigAmpSweep>,
) -> Result {
    let (Some(rfe), Some(config)) = (rfe, config) else {
        return Result::NullPtrError;
    };

    if let Some(config_amp_sweep) = rfe.config_amp_sweep() {
        *config = SignalGeneratorConfigAmpSweep::from(config_amp_sweep);
        Result::Success
    } else {
        Result::NoData
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_config_cw(
    rfe: Option<&SignalGenerator>,
    config: Option<&mut SignalGeneratorConfigCw>,
) -> Result {
    let (Some(rfe), Some(config)) = (rfe, config) else {
        return Result::NullPtrError;
    };

    if let Some(config_cw) = rfe.config_cw() {
        *config = SignalGeneratorConfigCw::from(config_cw);
        Result::Success
    } else {
        Result::NoData
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_config_freq_sweep(
    rfe: Option<&SignalGenerator>,
    config: Option<&mut SignalGeneratorConfigFreqSweep>,
) -> Result {
    let (Some(rfe), Some(config)) = (rfe, config) else {
        return Result::NullPtrError;
    };

    if let Some(config_freq_sweep) = rfe.config_freq_sweep() {
        *config = SignalGeneratorConfigFreqSweep::from(config_freq_sweep);
        Result::Success
    } else {
        Result::NoData
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_signal_generator_screen_data(
    rfe: Option<&SignalGenerator>,
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
pub extern "C" fn rfe_signal_generator_wait_for_next_screen_data(
    rfe: Option<&SignalGenerator>,
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
pub extern "C" fn rfe_signal_generator_wait_for_next_screen_data_with_timeout(
    rfe: Option<&SignalGenerator>,
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
pub extern "C" fn rfe_signal_generator_temperature(
    rfe: Option<&SignalGenerator>,
    temperature: Option<&mut Temperature>,
) -> Result {
    let (Some(rfe), Some(temperature)) = (rfe, temperature) else {
        return Result::NullPtrError;
    };

    if let Some(temp) = rfe.temperature() {
        *temperature = temp;
        Result::Success
    } else {
        Result::NoData
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_main_radio_model(
    rfe: Option<&SignalGenerator>,
    model: Option<&mut SignalGeneratorModel>,
) -> Result {
    let (Some(rfe), Some(model)) = (rfe, model) else {
        return Result::NullPtrError;
    };

    if let Some(main_model) = rfe.main_radio_model() {
        *model = main_model.into();
        Result::Success
    } else {
        Result::NoData
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_expansion_radio_model(
    rfe: Option<&SignalGenerator>,
    model: Option<&mut SignalGeneratorModel>,
) -> Result {
    let (Some(rfe), Some(model)) = (rfe, model) else {
        return Result::NullPtrError;
    };

    if let Some(expansion_model) = rfe.expansion_radio_model() {
        *model = expansion_model.into();
        Result::Success
    } else {
        Result::NoData
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_active_radio_model(
    rfe: Option<&SignalGenerator>,
    model: Option<&mut SignalGeneratorModel>,
) -> Result {
    if let (Some(rfe), Some(model)) = (rfe, model) {
        *model = rfe.active_radio_model().into();
        Result::Success
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_inactive_radio_model(
    rfe: Option<&SignalGenerator>,
    model: Option<&mut SignalGeneratorModel>,
) -> Result {
    let (Some(rfe), Some(model)) = (rfe, model) else {
        return Result::NullPtrError;
    };

    if let Some(inactive_model) = rfe.inactive_radio_model() {
        *model = inactive_model.into();
        Result::Success
    } else {
        Result::NoData
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_start_amp_sweep(
    rfe: Option<&SignalGenerator>,
    cw_hz: u64,
    start_attenuation: Attenuation,
    start_power_level: PowerLevel,
    stop_attenuation: Attenuation,
    stop_power_level: PowerLevel,
    step_delay_sec: u8,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.start_amp_sweep(
            cw_hz,
            start_attenuation,
            start_power_level,
            stop_attenuation,
            stop_power_level,
            Duration::from_secs(u64::from(step_delay_sec)),
        )
        .into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_start_amp_sweep_exp(
    rfe: Option<&SignalGenerator>,
    cw_hz: u64,
    start_power_dbm: f64,
    step_power_db: f64,
    stop_power_dbm: f64,
    step_delay_sec: u8,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.start_amp_sweep_exp(
            cw_hz,
            start_power_dbm,
            step_power_db,
            stop_power_dbm,
            Duration::from_secs(u64::from(step_delay_sec)),
        )
        .into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_start_cw(
    rfe: Option<&SignalGenerator>,
    cw_hz: u64,
    attenuation: Attenuation,
    power_level: PowerLevel,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.start_cw(cw_hz, attenuation, power_level).into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_start_cw_exp(
    rfe: Option<&SignalGenerator>,
    cw_hz: u64,
    power_dbm: f64,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.start_cw_exp(cw_hz, power_dbm).into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_start_freq_sweep(
    rfe: Option<&SignalGenerator>,
    start_hz: u64,
    attenuation: Attenuation,
    power_level: PowerLevel,
    sweep_steps: u16,
    step_hz: u64,
    step_delay_sec: u8,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.start_freq_sweep(
            start_hz,
            attenuation,
            power_level,
            sweep_steps,
            step_hz,
            Duration::from_secs(u64::from(step_delay_sec)),
        )
        .into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_start_freq_sweep_exp(
    rfe: Option<&SignalGenerator>,
    start_hz: u64,
    power_dbm: f64,
    sweep_steps: u16,
    step_hz: u64,
    step_delay_sec: u8,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.start_freq_sweep_exp(
            start_hz,
            power_dbm,
            sweep_steps,
            step_hz,
            Duration::from_secs(u64::from(step_delay_sec)),
        )
        .into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_start_tracking(
    rfe: Option<&SignalGenerator>,
    start_hz: u64,
    attenuation: Attenuation,
    power_level: PowerLevel,
    sweep_steps: u16,
    step_hz: u64,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.start_tracking(start_hz, attenuation, power_level, sweep_steps, step_hz)
            .into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_start_tracking_exp(
    rfe: Option<&SignalGenerator>,
    start_hz: u64,
    power_dbm: f64,
    sweep_steps: u16,
    step_hz: u64,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.start_tracking_exp(start_hz, power_dbm, sweep_steps, step_hz)
            .into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_tracking_step(
    rfe: Option<&SignalGenerator>,
    steps: u16,
) -> Result {
    if let Some(rfe) = rfe {
        rfe.tracking_step(steps).into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_set_config_callback(
    rfe: Option<&SignalGenerator>,
    callback: Option<extern "C" fn(config: SignalGeneratorConfig, user_data: *mut c_void)>,
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
        callback(SignalGeneratorConfig::from(config), user_data.clone().0);
    };

    rfe.set_config_callback(cb);
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_remove_config_callback(rfe: Option<&SignalGenerator>) {
    if let Some(rfe) = rfe {
        rfe.remove_config_callback();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_set_config_amp_sweep_callback(
    rfe: Option<&SignalGenerator>,
    callback: Option<extern "C" fn(config: SignalGeneratorConfigAmpSweep, user_data: *mut c_void)>,
    user_data: *mut c_void,
) {
    let (Some(rfe), Some(callback)) = (rfe, callback) else {
        return;
    };

    // Wrap the pointer to user_data in our own struct that implements Send so it can be
    // sent across threads
    let user_data = UserDataWrapper(user_data);

    // Convert the C function pointer to a Rust closure
    let cb = move |config: ConfigAmpSweep| {
        callback(
            SignalGeneratorConfigAmpSweep::from(config),
            user_data.clone().0,
        );
    };

    rfe.set_config_amp_sweep_callback(cb);
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_remove_config_amp_sweep_callback(
    rfe: Option<&SignalGenerator>,
) {
    if let Some(rfe) = rfe {
        rfe.remove_config_amp_sweep_callback();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_set_config_cw_callback(
    rfe: Option<&SignalGenerator>,
    callback: Option<extern "C" fn(config: SignalGeneratorConfigCw, user_data: *mut c_void)>,
    user_data: *mut c_void,
) {
    let (Some(rfe), Some(callback)) = (rfe, callback) else {
        return;
    };

    // Wrap the pointer to user_data in our own struct that implements Send so it can be
    // sent across threads
    let user_data = UserDataWrapper(user_data);

    // Convert the C function pointer to a Rust closure
    let cb = move |config: ConfigCw| {
        callback(SignalGeneratorConfigCw::from(config), user_data.clone().0);
    };

    rfe.set_config_cw_callback(cb);
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_remove_config_cw_callback(rfe: Option<&SignalGenerator>) {
    if let Some(rfe) = rfe {
        rfe.remove_config_cw_callback();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_set_config_freq_sweep_callback(
    rfe: Option<&SignalGenerator>,
    callback: Option<extern "C" fn(config: SignalGeneratorConfigFreqSweep, user_data: *mut c_void)>,
    user_data: *mut c_void,
) {
    let (Some(rfe), Some(callback)) = (rfe, callback) else {
        return;
    };

    // Wrap the pointer to user_data in our own struct that implements Send so it can be
    // sent across threads
    let user_data = UserDataWrapper(user_data);

    // Convert the C function pointer to a Rust closure
    let cb = move |config: ConfigFreqSweep| {
        callback(
            SignalGeneratorConfigFreqSweep::from(config),
            user_data.clone().0,
        );
    };

    rfe.set_config_freq_sweep_callback(cb);
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_remove_config_freq_sweep_callback(
    rfe: Option<&SignalGenerator>,
) {
    if let Some(rfe) = rfe {
        rfe.remove_config_freq_sweep_callback();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_rf_power_on(rfe: Option<&SignalGenerator>) -> Result {
    if let Some(rfe) = rfe {
        rfe.rf_power_on().into()
    } else {
        Result::NullPtrError
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rfe_signal_generator_rf_power_off(rfe: Option<&SignalGenerator>) -> Result {
    if let Some(rfe) = rfe {
        rfe.rf_power_off().into()
    } else {
        Result::NullPtrError
    }
}
