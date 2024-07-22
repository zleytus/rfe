use rfe::{Frequency, SpectrumAnalyzer};

use crate::Units;

/// The settings of an RF Explorer's sweep.
#[derive(Debug, Clone)]
pub struct SweepSettings {
    pub center_freq: String,
    pub span: String,
    pub start_freq: String,
    pub stop_freq: String,
    pub rbw: Option<Frequency>,
    pub step_size: Frequency,
    pub len: u16,
}

impl SweepSettings {
    pub fn new(rfe: Option<&SpectrumAnalyzer>, units: Units) -> Self {
        let start_freq = rfe.map(SpectrumAnalyzer::start_freq).unwrap_or_default();
        let stop_freq = rfe.map(SpectrumAnalyzer::stop_freq).unwrap_or_default();
        let center_freq = rfe.map(SpectrumAnalyzer::center_freq).unwrap_or_default();
        let span = rfe.map(SpectrumAnalyzer::span).unwrap_or_default();
        let rbw = rfe.and_then(SpectrumAnalyzer::rbw);
        let step_size = rfe.map(SpectrumAnalyzer::step_size).unwrap_or_default();
        let len = rfe.map(SpectrumAnalyzer::sweep_len).unwrap_or_default();
        match units {
            Units::Hz => SweepSettings {
                start_freq: start_freq.as_hz().to_string(),
                stop_freq: stop_freq.as_hz().to_string(),
                center_freq: center_freq.as_hz().to_string(),
                span: span.as_hz().to_string(),
                rbw,
                step_size,
                len,
            },
            Units::Khz => SweepSettings {
                start_freq: start_freq.as_khz_f64().to_string(),
                stop_freq: stop_freq.as_khz_f64().to_string(),
                center_freq: center_freq.as_khz_f64().to_string(),
                span: span.as_khz_f64().to_string(),
                rbw,
                step_size,
                len,
            },
            Units::Mhz => SweepSettings {
                start_freq: format!("{:.2}", start_freq.as_mhz_f64()),
                stop_freq: format!("{:.2}", stop_freq.as_mhz_f64()),
                center_freq: format!("{:.2}", center_freq.as_mhz_f64()),
                span: format!("{:.2}", span.as_mhz_f64()),
                rbw,
                step_size,
                len,
            },
            Units::Ghz => SweepSettings {
                start_freq: format!("{:.5}", start_freq.as_ghz_f64()),
                stop_freq: format!("{:.5}", stop_freq.as_ghz_f64()),
                center_freq: format!("{:.5}", center_freq.as_ghz_f64()),
                span: format!("{:.5}", span.as_ghz_f64()),
                rbw,
                step_size,
                len,
            },
        }
    }
}

impl Default for SweepSettings {
    fn default() -> Self {
        SweepSettings {
            center_freq: "0".to_string(),
            span: "0".to_string(),
            start_freq: "0".to_string(),
            stop_freq: "0".to_string(),
            rbw: Some(Frequency::default()),
            step_size: Frequency::default(),
            len: u16::default(),
        }
    }
}
