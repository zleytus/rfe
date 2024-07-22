use std::{
    default::Default,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::{
    panels::{BottomPanel, PlotCentralPanel, PlotSettingsPanel, RfeSettingsPanel},
    settings::{PlotSettings, RfeSettings, Units},
    Sweeps,
};

#[derive(Debug)]
pub struct App {
    pub rfe: Option<rfe::SpectrumAnalyzer>,
    pub show_rfe_settings: bool,
    pub rfe_settings: RfeSettings,
    pub show_plot_settings: bool,
    plot_settings: PlotSettings,
    pub paused: bool,
    rfe_settings_changed: Arc<AtomicBool>,
    sweeps: Option<Sweeps>,
    sweep_changed: Arc<AtomicBool>,
    pub units: Units,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, rfe: Option<rfe::SpectrumAnalyzer>) -> Self {
        // Register a callback that sets the `rfe_settings_changed` flag to true when the RF Explorer's
        // settings change
        let rfe_settings_changed = Arc::new(AtomicBool::new(true));
        let rfe_settings_changed_clone = rfe_settings_changed.clone();
        let ctx = cc.egui_ctx.clone();
        if let Some(ref rfe) = rfe {
            rfe.set_config_callback(move || {
                rfe_settings_changed_clone.store(true, Ordering::Relaxed);
                ctx.request_repaint();
            });
        }

        // Register a callback that sets the `sweep_changed` flag to true when the RF Explorer receives
        // a new sweep
        let sweep_changed = Arc::new(AtomicBool::new(false));
        let sweep_changed_clone = sweep_changed.clone();
        let ctx = cc.egui_ctx.clone();
        if let Some(ref rfe) = rfe {
            rfe.set_sweep_callback(move |_| {
                sweep_changed_clone.store(true, Ordering::Relaxed);
                ctx.request_repaint();
            });
        }

        let units = Units::Mhz;
        let rfe_settings = RfeSettings::new(rfe.as_ref(), units);

        Self {
            rfe,
            show_rfe_settings: true,
            rfe_settings,
            show_plot_settings: false,
            plot_settings: PlotSettings::default(),
            paused: false,
            rfe_settings_changed,
            sweeps: None,
            sweep_changed,
            units,
        }
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Recreate our records of RF Explorer's settings and sweeps when the `rfe_settings_changed`
        // flag is true
        if self.rfe_settings_changed.load(Ordering::Relaxed) {
            self.sweeps = self.rfe.as_ref().map(|rfe| Sweeps::new(rfe));
            self.rfe_settings = RfeSettings::new(self.rfe.as_ref(), self.units);
            self.rfe_settings_changed.store(false, Ordering::Relaxed);
        }

        // Update our record of RF Explorer's sweeps when the `sweep_changed` flag is true
        if !self.paused && self.sweep_changed.load(Ordering::Relaxed) {
            if let (Some(rfe), Some(sweeps)) = (&self.rfe, &mut self.sweeps) {
                if let Some(amps) = &rfe.sweep() {
                    sweeps.update(amps);
                }
            }
            self.sweep_changed
                .store(false, std::sync::atomic::Ordering::Relaxed);
        }

        BottomPanel::new().show(ctx, self);

        if self.show_rfe_settings {
            RfeSettingsPanel::new().show(
                ctx,
                self.rfe.as_ref(),
                &mut self.rfe_settings,
                self.units,
            );
        }

        if self.show_plot_settings {
            PlotSettingsPanel::new().show(
                ctx,
                &mut self.plot_settings,
                self.sweeps.as_mut(),
                self.units,
            );
        }

        PlotCentralPanel::new().show(ctx, self.sweeps.as_ref(), &self.plot_settings, self.units);
    }
}
