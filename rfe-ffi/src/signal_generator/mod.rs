mod config;
mod list;
mod rf_explorer;

use config::{
    SignalGeneratorConfig, SignalGeneratorConfigAmpSweep, SignalGeneratorConfigCw,
    SignalGeneratorConfigFreqSweep,
};
use list::SignalGeneratorList;

type SignalGenerator = rfe::RfExplorer<rfe::SignalGenerator>;
