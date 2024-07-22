use rfe::{Frequency, SpectrumAnalyzer};

use super::Units;

#[derive(Debug, Default)]
pub struct RfeSettings {
    pub sweep_settings: SweepSettings,
    pub rfe_info: RfeInfo,
}

impl RfeSettings {
    pub fn new(rfe: Option<&SpectrumAnalyzer>, units: Units) -> Self {
        rfe.map(|rfe| Self {
            sweep_settings: SweepSettings::new(rfe, units),
            rfe_info: RfeInfo::new(rfe),
        })
        .unwrap_or_default()
    }
}

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
    pub fn new(rfe: &SpectrumAnalyzer, units: Units) -> Self {
        match units {
            Units::Hz => SweepSettings {
                start_freq: rfe.start_freq().as_hz().to_string(),
                stop_freq: rfe.stop_freq().as_hz().to_string(),
                center_freq: rfe.center_freq().as_hz().to_string(),
                span: rfe.span().as_hz().to_string(),
                rbw: rfe.rbw(),
                step_size: rfe.step_size(),
                len: rfe.sweep_len(),
            },
            Units::Khz => SweepSettings {
                start_freq: rfe.start_freq().as_khz_f64().to_string(),
                stop_freq: rfe.stop_freq().as_khz_f64().to_string(),
                center_freq: rfe.center_freq().as_khz_f64().to_string(),
                span: rfe.span().as_khz_f64().to_string(),
                rbw: rfe.rbw(),
                step_size: rfe.step_size(),
                len: rfe.sweep_len(),
            },
            Units::Mhz => SweepSettings {
                start_freq: format!("{:.2}", rfe.start_freq().as_mhz_f64()),
                stop_freq: format!("{:.2}", rfe.stop_freq().as_mhz_f64()),
                center_freq: format!("{:.2}", rfe.center_freq().as_mhz_f64()),
                span: format!("{:.2}", rfe.span().as_mhz_f64()),
                rbw: rfe.rbw(),
                step_size: rfe.step_size(),
                len: rfe.sweep_len(),
            },
            Units::Ghz => SweepSettings {
                start_freq: format!("{:.5}", rfe.start_freq().as_ghz_f64()),
                stop_freq: format!("{:.5}", rfe.stop_freq().as_ghz_f64()),
                center_freq: format!("{:.5}", rfe.center_freq().as_ghz_f64()),
                span: format!("{:.5}", rfe.span().as_ghz_f64()),
                rbw: rfe.rbw(),
                step_size: rfe.step_size(),
                len: rfe.sweep_len(),
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
    pub fn new(rfe: &SpectrumAnalyzer) -> Self {
        Self {
            min_freq: rfe.min_freq(),
            max_freq: rfe.max_freq(),
            max_span: rfe.max_span(),
            active_radio_model: rfe.active_radio_model().to_string(),
            inactive_radio_model: rfe.inactive_radio_model().map(|model| model.to_string()),
            calc_mode: rfe.calc_mode().map(|calc_mode| calc_mode.to_string()),
            input_stage: rfe.input_stage().map(|input_stage| input_stage.to_string()),
            dsp_mode: rfe.dsp_mode().map(|dsp_mode| dsp_mode.to_string()),
            port_name: rfe.port_name().to_string(),
            firmware_version: rfe.firmware_version(),
            serial_number: rfe.serial_number(),
        }
    }
}
