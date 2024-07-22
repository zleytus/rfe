use std::sync::{atomic::AtomicBool, Arc};

use super::FrequencyUnits;

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub show_rfe_settings_panel: bool,
    pub show_plot_settings_panel: bool,
    pub pause_sweeps: Arc<AtomicBool>,
    pub frequency_units: FrequencyUnits,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            show_rfe_settings_panel: true,
            show_plot_settings_panel: true,
            pause_sweeps: Arc::new(AtomicBool::new(false)),
            frequency_units: FrequencyUnits::Mhz,
        }
    }
}
