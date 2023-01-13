mod config;
mod list;
mod rf_explorer;
mod sweep;

use config::SpectrumAnalyzerConfig;
use list::SpectrumAnalyzerList;
use sweep::Sweep;

type SpectrumAnalyzer = rfe::RfExplorer<rfe::SpectrumAnalyzer>;
