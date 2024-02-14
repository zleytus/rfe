mod list;
mod model;
mod radio_module;
mod rf_explorer;

use list::SpectrumAnalyzerList;
use radio_module::SpectrumAnalyzerRadioModule;

type SpectrumAnalyzer = rfe::spectrum_analyzer::RfExplorer;
