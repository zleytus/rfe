use egui::{CentralPanel, Context, InnerResponse};

use crate::{widgets::Plot, PlotSettings, Sweeps, Units};

pub struct PlotCentralPanel {
    panel: CentralPanel,
}

impl PlotCentralPanel {
    pub fn new() -> Self {
        Self {
            panel: CentralPanel::default(),
        }
    }

    pub fn show(
        self,
        ctx: &Context,
        sweeps: &Sweeps,
        plot_settings: &PlotSettings,
        units: Units,
    ) -> InnerResponse<()> {
        self.panel.show(ctx, |ui| {
            Plot::show(ui, sweeps, plot_settings, units);
        })
    }
}
