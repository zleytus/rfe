mod config;
mod list;
mod model;
mod radio_module;
mod rf_explorer;

use config::{
    SignalGeneratorConfig, SignalGeneratorConfigAmpSweep, SignalGeneratorConfigCw,
    SignalGeneratorConfigFreqSweep,
};
use list::SignalGeneratorList;
use model::SignalGeneratorModel;
use radio_module::SignalGeneratorRadioModule;

type SignalGenerator = rfe::signal_generator::RfExplorer;
