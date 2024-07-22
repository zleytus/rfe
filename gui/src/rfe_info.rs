use rfe::{Frequency, SpectrumAnalyzer};

#[derive(Debug, Default)]
pub struct RfeInfo {
    pub min_freq: Frequency,
    pub max_freq: Frequency,
    pub max_span: Frequency,
    pub active_radio_model: String,
    pub inactive_radio_model: Option<String>,
    pub calc_mode: Option<String>,
    pub input_stage: Option<String>,
    pub dsp_mode: Option<String>,
    pub port_name: String,
    pub firmware_version: String,
    pub serial_number: Option<String>,
}

impl RfeInfo {
    pub fn new(rfe: Option<&SpectrumAnalyzer>) -> Self {
        Self {
            min_freq: rfe.map(SpectrumAnalyzer::min_freq).unwrap_or_default(),
            max_freq: rfe.map(SpectrumAnalyzer::max_freq).unwrap_or_default(),
            max_span: rfe.map(SpectrumAnalyzer::max_span).unwrap_or_default(),
            active_radio_model: rfe
                .map(|rfe| rfe.active_radio_model().to_string())
                .unwrap_or_else(|| "---------".to_string()),
            inactive_radio_model: rfe
                .and_then(SpectrumAnalyzer::inactive_radio_model)
                .map(|model| model.to_string()),
            calc_mode: rfe
                .and_then(SpectrumAnalyzer::calc_mode)
                .map(|calc_mode| calc_mode.to_string()),
            input_stage: rfe
                .and_then(SpectrumAnalyzer::input_stage)
                .map(|input_stage| input_stage.to_string()),
            dsp_mode: rfe
                .and_then(SpectrumAnalyzer::dsp_mode)
                .map(|dsp_mode| dsp_mode.to_string()),
            port_name: rfe
                .map(SpectrumAnalyzer::port_name)
                .unwrap_or("---------")
                .to_string(),
            firmware_version: rfe
                .map(SpectrumAnalyzer::firmware_version)
                .unwrap_or_else(|| "---------".to_string()),
            serial_number: rfe.and_then(SpectrumAnalyzer::serial_number),
        }
    }
}
