use rfe::RadioModule;

use super::model::SpectrumAnalyzerModel;

#[repr(C)]
pub struct SpectrumAnalyzerRadioModule {
    pub model: SpectrumAnalyzerModel,
    pub is_expansion_radio_module: bool,
}

impl From<RadioModule> for SpectrumAnalyzerRadioModule {
    fn from(radio_module: RadioModule) -> Self {
        match radio_module {
            RadioModule::Main { model } => Self {
                model: SpectrumAnalyzerModel::from(model),
                is_expansion_radio_module: false,
            },
            RadioModule::Expansion { model } => Self {
                model: SpectrumAnalyzerModel::from(model),
                is_expansion_radio_module: true,
            },
        }
    }
}
