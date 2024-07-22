use rfe::{spectrum_analyzer::Config, Frequency, SpectrumAnalyzer};

use super::FrequencyUnits;

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
    units: FrequencyUnits,
}

impl SweepSettings {
    pub fn new(rfe: &SpectrumAnalyzer, units: FrequencyUnits) -> Self {
        Self {
            start_freq: freq_to_string(rfe.start_freq(), units),
            stop_freq: freq_to_string(rfe.stop_freq(), units),
            center_freq: freq_to_string(rfe.center_freq(), units),
            span: freq_to_string(rfe.span(), units),
            rbw: rfe.rbw(),
            step_size: rfe.step_size(),
            len: rfe.sweep_len(),
            units,
        }
    }

    pub fn update(&mut self, config: &Config) {
        self.start_freq = freq_to_string(config.start_freq, self.units);
        self.stop_freq = freq_to_string(config.stop_freq, self.units);
        self.center_freq = freq_to_string(config.center_freq, self.units);
        self.span = freq_to_string(config.span, self.units);
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
            units: FrequencyUnits::Mhz,
        }
    }
}

fn freq_to_string(freq: Frequency, units: FrequencyUnits) -> String {
    match units {
        FrequencyUnits::Hz => freq.as_hz().to_string(),
        FrequencyUnits::Khz => freq.as_khz_f64().to_string(),
        FrequencyUnits::Mhz => format!("{:.2}", freq.as_mhz_f64()),
        FrequencyUnits::Ghz => format!("{:.5}", freq.as_ghz_f64()),
    }
}
