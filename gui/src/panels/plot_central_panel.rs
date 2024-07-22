use egui::{CentralPanel, Context, TopBottomPanel};

use crate::{
    data::{SpectrogramData, TraceData},
    settings::{FrequencyUnits, SpectrogramSettings, TraceSettings},
    widgets::{Spectrogram, Trace},
};

pub struct PlotCentralPanel {
    central_panel: CentralPanel,
    bottom_panel: TopBottomPanel,
}

impl PlotCentralPanel {
    pub fn new() -> Self {
        Self {
            central_panel: CentralPanel::default(),
            bottom_panel: TopBottomPanel::bottom("spectrogram-plot-panel")
                .resizable(true)
                .default_height(250.0),
        }
    }

    pub fn show(
        self,
        ctx: &Context,
        trace_data: &TraceData,
        trace_settings: &TraceSettings,
        spectrogram_data: &mut SpectrogramData,
        spectrogram_settings: &SpectrogramSettings,
        units: FrequencyUnits,
    ) {
        // Only put the spectrogram in the bottom panel if the trace is being shown in the central panel
        if !spectrogram_settings.hide_spectrogram && !trace_settings.hide_trace {
            self.bottom_panel.show(ctx, |ui| {
                Spectrogram::show(ui, spectrogram_data, units);
            });
        }

        self.central_panel.show(ctx, |ui| {
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
