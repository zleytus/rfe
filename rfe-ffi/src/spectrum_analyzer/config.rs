use rfe::spectrum_analyzer::{CalcMode, Mode, RadioModule};

#[repr(C)]
pub struct SpectrumAnalyzerConfig {
    start_hz: u64,
    step_hz: u64,
    stop_hz: u64,
    center_hz: u64,
    span_hz: u64,
    min_amp_dbm: i16,
    max_amp_dbm: i16,
    sweep_points: u16,
    active_radio_module: RadioModule,
    mode: Mode,
    min_freq_hz: u64,
    max_freq_hz: u64,
    max_span_hz: u64,
    rbw_hz: u64,
    amp_offset_db: i8,
    calc_mode: CalcMode,
}

impl From<rfe::spectrum_analyzer::Config> for SpectrumAnalyzerConfig {
    fn from(config: rfe::spectrum_analyzer::Config) -> Self {
        SpectrumAnalyzerConfig {
            start_hz: config.start.as_hz(),
            step_hz: config.step.as_hz(),
            stop_hz: config.stop.as_hz(),
            center_hz: config.center.as_hz(),
            span_hz: config.span.as_hz(),
            min_amp_dbm: config.min_amp_dbm,
            max_amp_dbm: config.max_amp_dbm,
            sweep_points: config.sweep_points,
            active_radio_module: config.active_radio_module,
            mode: config.mode,
            min_freq_hz: config.min_freq.as_hz(),
            max_freq_hz: config.max_freq.as_hz(),
            max_span_hz: config.max_span.as_hz(),
            rbw_hz: config.rbw.unwrap_or_default().as_hz(),
            amp_offset_db: config.amp_offset_db.unwrap_or_default(),
            calc_mode: config.calc_mode.unwrap_or_default(),
        }
    }
}
