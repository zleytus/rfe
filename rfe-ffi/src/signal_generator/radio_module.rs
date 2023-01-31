use rfe::{signal_generator::Model, RadioModule};

use super::SignalGeneratorModel;

#[repr(C)]
pub struct SignalGeneratorRadioModule {
    pub model: SignalGeneratorModel,
    pub is_expansion_radio_module: bool,
}

impl From<RadioModule<Model>> for SignalGeneratorRadioModule {
    fn from(radio_module: RadioModule<Model>) -> Self {
        match radio_module {
            RadioModule::Main { model } => Self {
                model: SignalGeneratorModel::from(model),
                is_expansion_radio_module: false,
            },
            RadioModule::Expansion { model } => Self {
                model: SignalGeneratorModel::from(model),
                is_expansion_radio_module: true,
            },
        }
    }
}
