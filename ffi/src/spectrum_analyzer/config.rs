use rfe::spectrum_analyzer::{CalcMode, Config, Mode};

#[repr(C)]
pub struct SpectrumAnalyzerConfig {
    start_freq_hz: u64,
    step_size_hz: u64,
    stop_freq_hz: u64,
    center_freq_hz: u64,
    span_hz: u64,
    max_amp_dbm: i16,
    min_amp_dbm: i16,
    sweep_len: u16,
    is_expansion_radio_module_active: bool,
    mode: Mode,
    min_freq_hz: u64,
    max_freq_hz: u64,
    max_span_hz: u64,
    rbw_hz: u64,
    amp_offset_db: i8,
    calc_mode: CalcMode,
}

impl From<Config> for SpectrumAnalyzerConfig {
    fn from(config: Config) -> Self {
        SpectrumAnalyzerConfig {
            start_freq_hz: config.start_freq.as_hz(),
            step_size_hz: config.step_size.as_hz(),
            stop_freq_hz: config.stop_freq.as_hz(),
            center_freq_hz: config.center_freq.as_hz(),
            span_hz: config.span.as_hz(),
            max_amp_dbm: config.max_amp_dbm,
            min_amp_dbm: config.min_amp_dbm,
            sweep_len: config.sweep_len,
            is_expansion_radio_module_active: config.is_expansion_radio_module_active,
            mode: config.mode,
            min_freq_hz: config.min_freq.as_hz(),
            max_freq_hz: config.max_freq.as_hz(),
            max_span_hz: config.max_span.as_hz(),
            rbw_hz: config.rbw.map(|freq| freq.as_hz()).unwrap_or_default(),
            amp_offset_db: config.amp_offset_db.unwrap_or_default(),
            calc_mode: config.calc_mode.unwrap_or_default(),
        }
    }
}
