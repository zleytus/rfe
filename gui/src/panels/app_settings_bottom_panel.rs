use std::sync::atomic::Ordering;

use egui::{Align, Layout, Panel, Ui};

use crate::{
    settings::AppSettings,
    widgets::{
        PauseScanningButton, PlotSettingsToggleButton, ResumeScanningButton,
        RfeSettingsToggleButton, UnitsComboBox,
    },
};

pub struct AppSettingsBottomPanel {
    panel: Panel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppSettingsPanelResponse {
    FrequencyUnitsChanged,
    ExportAverageTraceClicked,
    ExportCurrentTraceClicked,
    ExportMaxTraceClicked,
}

impl AppSettingsBottomPanel {
    pub fn new() -> Self {
        Self {
            panel: Panel::bottom("bottom-panel").default_size(30.0),
        }
    }

    pub fn show(
        self,
        ui: &mut Ui,
        app_settings: &mut AppSettings,
    ) -> Option<AppSettingsPanelResponse> {
        self.panel
            .show_inside(ui, |ui| {
                ui.columns(2, |columns| {
                    columns[0].with_layout(Layout::left_to_right(Align::Center), |ui| {
                        show_bottom_left(ui, app_settings);
                    });
                    columns[1]
                        .with_layout(Layout::right_to_left(Align::Center), |ui| {
                            show_bottom_right(ui, app_settings)
                        })
                        .inner
                })
            })
            .inner
    }
}

fn show_bottom_left(ui: &mut Ui, app_settings: &mut AppSettings) {
    if ui
        .add(RfeSettingsToggleButton::new(
            app_settings.show_rfe_settings_panel,
        ))
        .clicked()
    {
        app_settings.show_rfe_settings_panel = !app_settings.show_rfe_settings_panel;
    }
    if app_settings.pause_sweeps.load(Ordering::Relaxed) {
        if ui.add(ResumeScanningButton).clicked() {
            app_settings.pause_sweeps.store(false, Ordering::Relaxed);
        }
    } else {
        if ui.add(PauseScanningButton).clicked() {
            app_settings.pause_sweeps.store(true, Ordering::Relaxed);
        }
    }
}

fn show_bottom_right(
    ui: &mut Ui,
    app_settings: &mut AppSettings,
) -> Option<AppSettingsPanelResponse> {
    if ui
        .add(PlotSettingsToggleButton::new(
            app_settings.show_plot_settings_panel,
        ))
        .clicked()
    {
        app_settings.show_plot_settings_panel = !app_settings.show_plot_settings_panel;
    }
    let mut response = None;
    if UnitsComboBox::show_ui(ui, &mut app_settings.frequency_units).is_some_and(|r| r.changed()) {
        response = Some(AppSettingsPanelResponse::FrequencyUnitsChanged);
    }
    ui.menu_button("Export Trace as CSV...", |ui| {
        if ui.button("Average").clicked() {
            response = Some(AppSettingsPanelResponse::ExportAverageTraceClicked);
            ui.close();
        }
        if ui.button("Current").clicked() {
            response = Some(AppSettingsPanelResponse::ExportCurrentTraceClicked);
            ui.close();
        }
        if ui.button("Max").clicked() {
            response = Some(AppSettingsPanelResponse::ExportMaxTraceClicked);
            ui.close();
        }
    });
    response
}
