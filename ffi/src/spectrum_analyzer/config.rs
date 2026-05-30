use rfe::spectrum_analyzer::{CalcMode, Config, Mode};

/// Spectrum analyzer configuration.
///
/// Frequencies are represented in hertz. Fields that are optional in the Rust
/// API use zero or the enum default when the device has not reported a value.
#[repr(C)]
pub struct SpectrumAnalyzerConfig {
    /// Sweep start frequency.
    start_freq_hz: u64,
    /// Frequency step between sweep points.
    step_size_hz: u64,
    /// Sweep stop frequency.
    stop_freq_hz: u64,
    /// Sweep center frequency.
    center_freq_hz: u64,
    /// Sweep span.
    span_hz: u64,
    /// Top displayed amplitude in dBm.
    max_amp_dbm: i16,
    /// Bottom displayed amplitude in dBm.
    min_amp_dbm: i16,
    /// Number of points in each sweep.
    sweep_len: u16,
    /// Whether the expansion radio module is active.
    is_expansion_radio_module_active: bool,
    /// Current operating mode.
    mode: Mode,
    /// Minimum supported frequency.
    min_freq_hz: u64,
    /// Maximum supported frequency.
    max_freq_hz: u64,
    /// Maximum supported span.
    max_span_hz: u64,
    /// Resolution bandwidth, or zero if it has not been reported by the device.
    rbw_hz: u64,
    /// Amplitude offset in dB, or zero if it has not been reported by the device.
    amp_offset_db: i8,
    /// Calculator mode, or the default value if it has not been reported by the device.
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
