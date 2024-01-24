use rfe::signal_generator::{
    Attenuation, Config, ConfigAmpSweep, ConfigCw, ConfigFreqSweep, PowerLevel, RfPower,
};

#[repr(C)]
pub struct SignalGeneratorConfig {
    start_hz: u64,
    cw_hz: u64,
    total_steps: u32,
    step_hz: u64,
    attenuation: Attenuation,
    power_level: PowerLevel,
    sweep_power_steps: u16,
    start_attenuation: Attenuation,
    start_power_level: PowerLevel,
    stop_attenuation: Attenuation,
    stop_power_level: PowerLevel,
    rf_power: RfPower,
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

#[repr(C)]
pub struct SignalGeneratorConfigAmpSweep {
    cw_hz: u64,
    sweep_power_steps: u16,
    start_attenuation: Attenuation,
    start_power_level: PowerLevel,
    stop_attenuation: Attenuation,
    stop_power_level: PowerLevel,
    rf_power: RfPower,
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

#[repr(C)]
pub struct SignalGeneratorConfigCw {
    cw_hz: u64,
    total_steps: u32,
    step_freq_hz: u64,
    attenuation: Attenuation,
    power_level: PowerLevel,
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

#[repr(C)]
pub struct SignalGeneratorConfigFreqSweep {
    start_hz: u64,
    total_steps: u32,
    step_hz: u64,
    attenuation: Attenuation,
    power_level: PowerLevel,
    rf_power: RfPower,
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
