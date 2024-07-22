use std::{
    default::Default,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use crate::{
    panels::{BottomPanel, PlotCentralPanel, PlotSettingsPanel, RfeSettingsPanel},
    PlotSettings, RfeInfo, SweepSettings, Sweeps, Units,
};

#[derive(Debug)]
pub struct App {
    pub rfe: Option<rfe::SpectrumAnalyzer>,
    pub show_rfe_settings: bool,
    pub rfe_info: RfeInfo,
    pub sweep_settings: SweepSettings,
    pub show_plot_settings: bool,
    plot_settings: PlotSettings,
    pub paused: Arc<AtomicBool>,
    rfe_settings_changed: Arc<AtomicBool>,
    sweeps: Arc<Mutex<Sweeps>>,
    pub units: Units,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, rfe: Option<rfe::SpectrumAnalyzer>) -> Self {
        let paused = Arc::new(AtomicBool::new(false));
        let paused_clone = paused.clone();
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
        let sweeps = Arc::new(Mutex::new(Sweeps::default()));
        let sweeps_clone = sweeps.clone();

        let ctx = cc.egui_ctx.clone();
        if let Some(ref rfe) = rfe {
            rfe.set_sweep_callback(move |amps, start_freq, stop_freq| {
                if !paused_clone.load(Ordering::Relaxed) {
                    sweeps_clone
                        .lock()
                        .unwrap()
                        .update(amps, start_freq, stop_freq);
                    ctx.request_repaint();
                }
            });
        }

        let units = Units::Mhz;
        let sweep_settings = SweepSettings::new(rfe.as_ref(), units);
        let rfe_info = RfeInfo::new(rfe.as_ref());

        Self {
            rfe,
            show_rfe_settings: true,
            show_plot_settings: false,
            plot_settings: PlotSettings::default(),
            sweep_settings,
            rfe_info,
            paused,
            rfe_settings_changed,
            sweeps,
            units,
        }
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update our record of RF Explorer's settings when the `rfe_settings_changed`
        // flag is true
        if self.rfe_settings_changed.load(Ordering::Relaxed) {
            self.sweep_settings = SweepSettings::new(self.rfe.as_ref(), self.units);
            self.rfe_info = RfeInfo::new(self.rfe.as_ref());
            self.rfe_settings_changed.store(false, Ordering::Relaxed);
        }

        BottomPanel::new().show(ctx, self);

        if self.show_rfe_settings {
            RfeSettingsPanel::new().show(
                ctx,
                self.rfe.as_ref(),
                &mut self.sweep_settings,
                &mut self.rfe_info,
                self.units,
            );
        }

        if self.show_plot_settings {
            PlotSettingsPanel::new().show(
                ctx,
                &mut self.plot_settings,
                &mut self.sweeps.lock().unwrap(),
                self.units,
            );
        }

        PlotCentralPanel::new().show(
            ctx,
            &self.sweeps.lock().unwrap(),
            &self.plot_settings,
            self.units,
        );
    }
}
