use egui::{CentralPanel, Panel, Ui};

use crate::{
    data::{SpectrogramData, TraceData},
    settings::{FrequencyUnits, SpectrogramSettings, TraceSettings},
    widgets::{Spectrogram, Trace},
};

pub struct PlotCentralPanel {
    central_panel: CentralPanel,
    bottom_panel: Panel,
}

impl PlotCentralPanel {
    pub fn new() -> Self {
        Self {
            central_panel: CentralPanel::default(),
            bottom_panel: Panel::bottom("spectrogram-plot-panel")
                .resizable(true)
                .default_size(250.0),
        }
    }

    pub fn show(
        self,
        ui: &mut Ui,
        trace_data: &TraceData,
        trace_settings: &TraceSettings,
        spectrogram_data: &mut SpectrogramData,
        spectrogram_settings: &SpectrogramSettings,
        units: FrequencyUnits,
    ) {
        // Only put the spectrogram in the bottom panel if the trace is being shown in the central panel
        if !spectrogram_settings.hide_spectrogram && !trace_settings.hide_trace {
            self.bottom_panel.show_inside(ui, |ui| {
                Spectrogram::show(ui, spectrogram_data, units);
            });
        }

        self.central_panel.show_inside(ui, |ui| {
            if !trace_settings.hide_trace {
                Trace::show(ui, trace_data, trace_settings, units);
            }
            // Put the spectrogram in the central panel if the trace is hidden
            if trace_settings.hide_trace && !spectrogram_settings.hide_spectrogram {
                Spectrogram::show(ui, spectrogram_data, units);
            }
        });
    }
}
