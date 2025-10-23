use std::sync::atomic::Ordering;

use egui::{Align, Context, Layout, TopBottomPanel, Ui, UiKind};

use crate::{
    settings::AppSettings,
    widgets::{
        PauseScanningButton, PlotSettingsToggleButton, ResumeScanningButton,
        RfeSettingsToggleButton, UnitsComboBox,
    },
};

pub struct AppSettingsBottomPanel {
    panel: TopBottomPanel,
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
            panel: TopBottomPanel::bottom("bottom-panel").default_height(30.0),
        }
    }

    pub fn show(
        self,
        ctx: &Context,
        app_settings: &mut AppSettings,
    ) -> Option<AppSettingsPanelResponse> {
        self.panel
            .show(ctx, |ui| {
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
        if ui.add(ResumeScanningButton::default()).clicked() {
            app_settings.pause_sweeps.store(false, Ordering::Relaxed);
        }
    } else {
        if ui.add(PauseScanningButton::default()).clicked() {
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
            ui.close_kind(UiKind::Menu);
        }
        if ui.button("Current").clicked() {
            response = Some(AppSettingsPanelResponse::ExportCurrentTraceClicked);
            ui.close_kind(UiKind::Menu);
        }
        if ui.button("Max").clicked() {
            response = Some(AppSettingsPanelResponse::ExportMaxTraceClicked);
            ui.close_kind(UiKind::Menu);
        }
    });
    response
}
