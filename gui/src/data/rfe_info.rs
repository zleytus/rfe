use rfe::{
    spectrum_analyzer::{CalcMode, Config, DspMode, InputStage, Model},
    Frequency, SpectrumAnalyzer,
};

/// Information about an RF Explorer device.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct RfeInfo {
    /// The minimum supported frequency of an RF Explorer.
    pub min_freq: Frequency,
    /// The maximum supported frequency of an RF Explorer.
    pub max_freq: Frequency,
    /// The maximum supported frequency span of an RF Explorer.
    pub max_span: Frequency,
    /// The model of an RF Explorer's active radio.
    pub active_radio_model: Model,
    /// The model of an RF Explorer's inactive radio.
    pub inactive_radio_model: Option<Model>,
    pub calc_mode: Option<CalcMode>,
    pub input_stage: Option<InputStage>,
    pub dsp_mode: Option<DspMode>,
    pub port_name: String,
    /// The firmware version of an RF Explorer.
    pub firmware_version: String,
    /// The serial number of an RF Explorer.
    pub serial_number: Option<String>,
    is_expansion_radio_active: bool,
}

impl RfeInfo {
    pub fn new(rfe: &SpectrumAnalyzer) -> Self {
        Self {
            min_freq: rfe.min_freq(),
            max_freq: rfe.max_freq(),
            max_span: rfe.max_span(),
            active_radio_model: rfe.active_radio_model(),
            inactive_radio_model: rfe.inactive_radio_model(),
            calc_mode: rfe.calc_mode(),
            input_stage: rfe.input_stage(),
            dsp_mode: rfe.dsp_mode(),
            port_name: rfe.port_name().to_string(),
            firmware_version: rfe.firmware_version(),
            serial_number: rfe.serial_number(),
            is_expansion_radio_active: Some(rfe.active_radio_model())
                == rfe.expansion_radio_model(),
        }
    }

    pub fn update(&mut self, config: &Config) {
        self.min_freq = config.min_freq;
        self.max_freq = config.max_freq;
        self.max_span = config.max_span;
        self.calc_mode = config.calc_mode;

        // Swap the active and inactive radio models if the status of the expansion radio module has changed
        let Some(inactive_radio_model) = self.inactive_radio_model.clone() else {
            return;
        };
        if self.is_expansion_radio_active != config.is_expansion_radio_module_active {
            self.inactive_radio_model = Some(self.active_radio_model.clone());
            self.active_radio_model = inactive_radio_model;
        }
    }
}
