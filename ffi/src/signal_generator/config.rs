use rfe::signal_generator::{
    Attenuation, Config, ConfigAmpSweep, ConfigCw, ConfigFreqSweep, PowerLevel, RfPower,
};

/// Signal generator configuration.
///
/// Frequencies are represented in hertz. Durations are represented in
/// milliseconds.
#[repr(C)]
pub struct SignalGeneratorConfig {
    /// Start frequency for frequency sweep and tracking modes.
    start_hz: u64,
    /// CW frequency.
    cw_hz: u64,
    /// Total number of sweep or tracking steps.
    total_steps: u32,
    /// Frequency increment per step.
    step_hz: u64,
    /// CW and frequency sweep attenuation setting.
    attenuation: Attenuation,
    /// CW and frequency sweep power level.
    power_level: PowerLevel,
    /// Number of amplitude sweep power steps.
    sweep_power_steps: u16,
    /// Amplitude sweep start attenuation setting.
    start_attenuation: Attenuation,
    /// Amplitude sweep start power level.
    start_power_level: PowerLevel,
    /// Amplitude sweep stop attenuation setting.
    stop_attenuation: Attenuation,
    /// Amplitude sweep stop power level.
    stop_power_level: PowerLevel,
    /// RF output power state.
    rf_power: RfPower,
    /// Delay between sweep steps.
    sweep_delay_ms: u64,
}

impl From<Config> for SignalGeneratorConfig {
    fn from(config: Config) -> Self {
        SignalGeneratorConfig {
            start_hz: config.start.as_hz(),
            cw_hz: config.cw.as_hz(),
            total_steps: config.total_steps,
            step_hz: config.step.as_hz(),
            attenuation: config.attenuation,
            power_level: config.power_level,
            sweep_power_steps: config.sweep_power_steps,
            start_attenuation: config.start_attenuation,
            start_power_level: config.start_power_level,
            stop_attenuation: config.stop_attenuation,
            stop_power_level: config.stop_power_level,
            rf_power: config.rf_power,
            sweep_delay_ms: config.sweep_delay.as_millis() as u64,
        }
    }
}

/// Signal generator amplitude sweep configuration.
///
/// Frequencies are represented in hertz. Durations are represented in
/// milliseconds.
#[repr(C)]
pub struct SignalGeneratorConfigAmpSweep {
    /// CW frequency used during the amplitude sweep.
    cw_hz: u64,
    /// Number of power steps in the sweep.
    sweep_power_steps: u16,
    /// Starting attenuation setting.
    start_attenuation: Attenuation,
    /// Starting output power level.
    start_power_level: PowerLevel,
    /// Stopping attenuation setting.
    stop_attenuation: Attenuation,
    /// Stopping output power level.
    stop_power_level: PowerLevel,
    /// RF output power state.
    rf_power: RfPower,
    /// Delay between amplitude sweep steps.
    sweep_delay_ms: u64,
}

impl From<ConfigAmpSweep> for SignalGeneratorConfigAmpSweep {
    fn from(config: ConfigAmpSweep) -> Self {
        SignalGeneratorConfigAmpSweep {
            cw_hz: config.cw.as_hz(),
            sweep_power_steps: config.sweep_power_steps,
            start_attenuation: config.start_attenuation,
            start_power_level: config.start_power_level,
            stop_attenuation: config.stop_attenuation,
            stop_power_level: config.stop_power_level,
            rf_power: config.rf_power,
            sweep_delay_ms: config.sweep_delay.as_millis() as u64,
        }
    }
}

/// Signal generator CW configuration.
///
/// Frequencies are represented in hertz.
#[repr(C)]
pub struct SignalGeneratorConfigCw {
    /// CW frequency.
    cw_hz: u64,
    /// Total number of configured steps.
    total_steps: u32,
    /// Frequency increment per step.
    step_freq_hz: u64,
    /// RF output attenuation setting.
    attenuation: Attenuation,
    /// RF output power level.
    power_level: PowerLevel,
    /// RF output power state.
    rf_power: RfPower,
}

impl From<ConfigCw> for SignalGeneratorConfigCw {
    fn from(config: ConfigCw) -> Self {
        SignalGeneratorConfigCw {
            cw_hz: config.cw.as_hz(),
            total_steps: config.total_steps,
            step_freq_hz: config.step_freq.as_hz(),
            attenuation: config.attenuation,
            power_level: config.power_level,
            rf_power: config.rf_power,
        }
    }
}

/// Signal generator frequency sweep configuration.
///
/// Frequencies are represented in hertz. Durations are represented in
/// milliseconds.
#[repr(C)]
pub struct SignalGeneratorConfigFreqSweep {
    /// Start frequency.
    start_hz: u64,
    /// Total number of sweep steps.
    total_steps: u32,
    /// Frequency increment per step.
    step_hz: u64,
    /// RF output attenuation setting.
    attenuation: Attenuation,
    /// RF output power level.
    power_level: PowerLevel,
    /// RF output power state.
    rf_power: RfPower,
    /// Delay between sweep steps.
    sweep_delay_ms: u64,
}

impl From<ConfigFreqSweep> for SignalGeneratorConfigFreqSweep {
    fn from(config: ConfigFreqSweep) -> Self {
        SignalGeneratorConfigFreqSweep {
            start_hz: config.start.as_hz(),
            total_steps: config.total_steps,
            step_hz: config.step.as_hz(),
            attenuation: config.attenuation,
            power_level: config.power_level,
            rf_power: config.rf_power,
            sweep_delay_ms: config.sweep_delay.as_millis() as u64,
        }
    }
}
