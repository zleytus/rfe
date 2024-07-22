use std::{
    default::Default,
    num::ParseFloatError,
    str::FromStr,
    sync::{atomic::Ordering, Arc, Mutex},
};

use csv::Writer;
use rfd::FileDialog;
use rfe::{spectrum_analyzer::Config, Frequency, SpectrumAnalyzer};

use crate::{
    data::{RfeInfo, SpectrogramData, TraceData},
    panels::{
        AppSettingsBottomPanel, AppSettingsPanelResponse, PlotCentralPanel,
        PlotSettingsPanelResponse, PlotSettingsSidePanel, RfeNotConnectedCentralPanel,
        RfeSettingsPanelResponse, RfeSettingsSidePanel,
    },
    settings::{AppSettings, FrequencyUnits, SpectrogramSettings, SweepSettings, TraceSettings},
};

pub struct App {
    rfe: Option<Arc<Mutex<SpectrumAnalyzer>>>,
    rfe_info: Arc<Mutex<RfeInfo>>,
    trace_data: Arc<Mutex<TraceData>>,
    spectrogram_data: Arc<Mutex<SpectrogramData>>,
    app_settings: AppSettings,
    sweep_settings: Arc<Mutex<SweepSettings>>,
    trace_settings: TraceSettings,
    spectrogram_settings: Arc<Mutex<SpectrogramSettings>>,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, rfe: Option<rfe::SpectrumAnalyzer>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let rfe_info = rfe.as_ref().map(RfeInfo::new).unwrap_or_default();
        let app_settings = AppSettings::default();
        let sweep_settings = rfe
            .as_ref()
            .map(|rfe| SweepSettings::new(rfe, app_settings.frequency_units))
            .unwrap_or_default();

        let app = App {
            rfe: rfe.map(|rfe| Arc::new(Mutex::new(rfe))),
            rfe_info: Arc::new(Mutex::new(rfe_info)),
            trace_data: Arc::new(Mutex::new(TraceData::default())),
            spectrogram_data: Arc::new(Mutex::new(SpectrogramData::new(&cc.egui_ctx))),
            app_settings,
            sweep_settings: Arc::new(Mutex::new(sweep_settings)),
            trace_settings: TraceSettings::default(),
            spectrogram_settings: Arc::new(Mutex::new(SpectrogramSettings::default())),
        };

        app.init_callbacks(&cc.egui_ctx);
        app
    }

    fn init_callbacks(&self, egui_ctx: &egui::Context) {
        let Some(ref rfe) = self.rfe else {
            return;
        };

        // Register a callback that updates our `SweepSettings` and `RfeInfo` when the RF Explorer's
        // config changes
        let sweep_settings_clone = self.sweep_settings.clone();
        let rfe_info_clone = self.rfe_info.clone();
        let ctx = egui_ctx.clone();
        rfe.lock()
            .unwrap()
            .set_config_callback(move |config: Config| {
                sweep_settings_clone.lock().unwrap().update(&config);
                rfe_info_clone.lock().unwrap().update(&config);
                ctx.request_repaint();
            });

        // Register a callback that updates our data for the trace and the spectrogram when we receive
        // a new sweep
        let trace_data_clone = self.trace_data.clone();
        let spectrogram_data_clone = self.spectrogram_data.clone();
        let spectrogram_settings_clone = self.spectrogram_settings.clone();
        let pause_sweeps_clone = self.app_settings.pause_sweeps.clone();
        let ctx = egui_ctx.clone();
        rfe.lock()
            .unwrap()
            .set_sweep_callback(move |amps, start_freq, stop_freq| {
                if !pause_sweeps_clone.load(Ordering::Relaxed) {
                    trace_data_clone
                        .lock()
                        .unwrap()
                        .update(amps, start_freq, stop_freq);
                    spectrogram_data_clone.lock().unwrap().update(
                        amps,
                        start_freq,
                        stop_freq,
                        spectrogram_settings_clone.lock().as_ref().unwrap(),
                    );
                    ctx.request_repaint();
                }
            });
    }

    fn on_rfe_settings_changed(&self, panel_response: RfeSettingsPanelResponse) {
        let Some(ref rfe) = self.rfe else {
            return;
        };
        // We clone the sweep settings here so that we don't hold on to the lock
        // which would cause a deadlock when the RF Explorer sends a new `Config`
        // and our config callback gets called
        let sweep_settings = self.sweep_settings.lock().unwrap().clone();
        let units = self.app_settings.frequency_units;
        match panel_response {
            RfeSettingsPanelResponse::CenterSpanChanged => {
                let center_freq = str_to_freq(&sweep_settings.center_freq, units);
                let span = str_to_freq(&sweep_settings.span, units);
                let (Ok(center), Ok(span)) = (center_freq, span) else {
                    return;
                };
                // Call rfe.set_center_span on a non-UI thread because it would cause
                // the UI to freeze while it waits for a response from the RF Explorer
                let rfe_clone = rfe.clone();
                std::thread::spawn(move || {
                    _ = rfe_clone.lock().unwrap().set_center_span(center, span);
                });
            }
            RfeSettingsPanelResponse::StartStopChanged => {
                let start_freq = str_to_freq(&sweep_settings.start_freq, units);
                let stop_freq = str_to_freq(&sweep_settings.stop_freq, units);
                let (Ok(start), Ok(stop)) = (start_freq, stop_freq) else {
                    return;
                };
                // Call rfe.set_start_stop on a non-UI thread because it would cause
                // the UI to freeze while it waits for a response from the RF Explorer
                let rfe_clone = rfe.clone();
                std::thread::spawn(move || {
                    _ = rfe_clone.lock().unwrap().set_start_stop(start, stop);
                });
            }
            RfeSettingsPanelResponse::SweepLenChanged => {
                let center_freq = str_to_freq(&sweep_settings.center_freq, units);
                let span = str_to_freq(&sweep_settings.span, units);
                let sweep_len = sweep_settings.len;
                let (Ok(center), Ok(span)) = (center_freq, span) else {
                    return;
                };
                // Call rfe.set_center_span_sweep_len on a non-UI thread because it would cause
                // the UI to freeze while it waits for a response from the RF Explorer
                let rfe_clone = rfe.clone();
                std::thread::spawn(move || {
                    _ = rfe_clone
                        .lock()
                        .unwrap()
                        .set_center_span_sweep_len(center, span, sweep_len);
                });
            }
        }
    }

    fn on_app_settings_changed(&self, panel_response: AppSettingsPanelResponse) {
        match panel_response {
            AppSettingsPanelResponse::ExportCurrentTraceClicked => export_csv(
                self.trace_data.lock().unwrap().current(),
                self.app_settings.frequency_units,
            ),
            AppSettingsPanelResponse::ExportAverageTraceClicked => export_csv(
                self.trace_data.lock().unwrap().average(),
                self.app_settings.frequency_units,
            ),
            AppSettingsPanelResponse::ExportMaxTraceClicked => export_csv(
                self.trace_data.lock().unwrap().max(),
                self.app_settings.frequency_units,
            ),
            AppSettingsPanelResponse::FrequencyUnitsChanged => {
                // If the units setting was changed, recreate our record of the RF Explorer's settings
                *self.sweep_settings.lock().unwrap() = self
                    .rfe
                    .as_ref()
                    .map(|rfe| {
                        SweepSettings::new(&rfe.lock().unwrap(), self.app_settings.frequency_units)
                    })
                    .unwrap_or_default()
            }
        }
    }

    fn on_plot_settings_changed(&self, panel_response: PlotSettingsPanelResponse) {
        match panel_response {
            PlotSettingsPanelResponse::SpectrogramSettingsChanged => {
                self.spectrogram_data
                    .lock()
                    .unwrap()
                    .recreate_image(&self.spectrogram_settings.lock().unwrap());
            }
            PlotSettingsPanelResponse::TraceSettingsChanged => (),
        }
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let panel_response = AppSettingsBottomPanel::new().show(ctx, &mut self.app_settings);
        if let Some(panel_response) = panel_response {
            self.on_app_settings_changed(panel_response);
        }

        if self.app_settings.show_rfe_settings_panel {
            let can_change_sweep_len = self
                .rfe_info
                .lock()
                .unwrap()
                .active_radio_model
                .is_plus_model();
            let panel_response = RfeSettingsSidePanel::new().show(
                ctx,
                can_change_sweep_len,
                &mut self.sweep_settings.lock().unwrap(),
                &mut self.rfe_info.lock().unwrap(),
                self.app_settings.frequency_units,
            );
            if let Some(panel_response) = panel_response {
                self.on_rfe_settings_changed(panel_response);
            }
        }

        if self.app_settings.show_plot_settings_panel {
            let panel_response = PlotSettingsSidePanel::new().show(
                ctx,
                &mut self.trace_settings,
                &mut self.spectrogram_settings.lock().unwrap(),
            );
            if let Some(panel_response) = panel_response {
                self.on_plot_settings_changed(panel_response);
            }
        }

        if self.rfe.is_some() {
            PlotCentralPanel::new().show(
                ctx,
                &self.trace_data.lock().unwrap(),
                &self.trace_settings,
                &mut self.spectrogram_data.lock().unwrap(),
                &self.spectrogram_settings.lock().unwrap(),
                self.app_settings.frequency_units,
            );
        } else {
            RfeNotConnectedCentralPanel::new().show(ctx, &mut self.rfe);
            // If an RF Explorer is now connected, set the required callbacks
            if self.rfe.is_some() {
                self.init_callbacks(ctx);
                *self.sweep_settings.lock().unwrap() = self
                    .rfe
                    .as_ref()
                    .map(|rfe| {
                        SweepSettings::new(&rfe.lock().unwrap(), self.app_settings.frequency_units)
                    })
                    .unwrap_or_default();
                *self.rfe_info.lock().unwrap() = self
                    .rfe
                    .as_ref()
                    .map(|rfe| RfeInfo::new(&rfe.lock().unwrap()))
                    .unwrap_or_default();
            }
        }
    }
}

fn str_to_freq(str: &str, units: FrequencyUnits) -> Result<Frequency, ParseFloatError> {
    Ok(match units {
        FrequencyUnits::Hz => Frequency::from_hz(f64::from_str(str)? as u64),
        FrequencyUnits::Khz => Frequency::from_khz_f64(f64::from_str(str)?),
        FrequencyUnits::Mhz => Frequency::from_mhz_f64(f64::from_str(str)?),
        FrequencyUnits::Ghz => Frequency::from_ghz_f64(f64::from_str(str)?),
    })
}

fn freq_to_string(freq: Frequency, units: FrequencyUnits) -> String {
    match units {
        FrequencyUnits::Hz => freq.as_hz().to_string(),
        FrequencyUnits::Khz => format!("{:.3}", freq.as_khz_f64()),
        FrequencyUnits::Mhz => format!("{:.3}", freq.as_mhz_f64()),
        FrequencyUnits::Ghz => format!("{:.3}", freq.as_ghz_f64()),
    }
}

fn export_csv(trace: &[(Frequency, f64)], units: FrequencyUnits) {
    if trace.is_empty() {
        return;
    }

    // Open the save file dialog in a new thread so we don't block the UI thread from updating
    let trace = trace.to_vec();
    std::thread::spawn(move || {
        let Some(Ok(mut writer)) = FileDialog::new()
            .set_title("Export CSV")
            .add_filter("CSV", &["csv"])
            .set_file_name("trace.csv")
            .save_file()
            .map(Writer::from_path)
        else {
            return;
        };
        for (freq, amp) in trace.iter().map(|point| (point.0, point.1)) {
            let record = [freq_to_string(freq, units), amp.to_string()];
            if writer.write_record(record).is_err() {
                break;
            }
        }
        _ = writer.flush();
    });
}
