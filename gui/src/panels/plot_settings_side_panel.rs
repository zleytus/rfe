use egui::{
    color_picker::{self, Alpha},
    Context, Image, ScrollArea, SidePanel, Slider, Ui,
};

use super::{Setting, SettingsCategory};
use crate::{
    settings::{SpectrogramSettings, TraceSettings},
    widgets::SpectrogramColorGradientComboBox,
};

pub struct PlotSettingsSidePanel {
    side_panel: SidePanel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlotSettingsPanelResponse {
    TraceSettingsChanged,
    SpectrogramSettingsChanged,
}

impl PlotSettingsSidePanel {
    pub fn new() -> Self {
        Self {
            side_panel: SidePanel::right("plot-settings-panel").resizable(false),
        }
    }

    pub fn show(
        self,
        ctx: &Context,
        trace_settings: &mut TraceSettings,
        spectrogram_settings: &mut SpectrogramSettings,
    ) -> Option<PlotSettingsPanelResponse> {
        // Save copies of the settings before they can be changed
        let old_trace_settings = trace_settings.clone();
        let old_spectrogram_settings = spectrogram_settings.clone();

        self.side_panel.show(ctx, |ui| {
            ScrollArea::vertical()
                .show(ui, |ui| {
                    ui.add_space(5.0);
                    show_trace_settings(ui, trace_settings);
                    ui.add_space(10.0);
                    show_spectrogram_settings(ui, spectrogram_settings);
                })
                .inner
        });

        // Check to see if the settings have been changed
        if old_trace_settings != *trace_settings {
            Some(PlotSettingsPanelResponse::TraceSettingsChanged)
        } else if old_spectrogram_settings != *spectrogram_settings {
            Some(PlotSettingsPanelResponse::SpectrogramSettingsChanged)
        } else {
            None
        }
    }
}

fn show_trace_settings(ui: &mut Ui, trace_settings: &mut TraceSettings) {
    SettingsCategory::new("Trace").show(ui, 6, |row| match row.index() {
        0 => {
            Setting::new("Line Colors", |ui| {
                color_picker::color_edit_button_srgba(
                    ui,
                    &mut trace_settings.max_trace_color,
                    Alpha::Opaque,
                )
                .on_hover_text("Max");
                color_picker::color_edit_button_srgba(
                    ui,
                    &mut trace_settings.current_trace_color,
                    Alpha::Opaque,
                )
                .on_hover_text("Current");
                color_picker::color_edit_button_srgba(
                    ui,
                    &mut trace_settings.average_trace_color,
                    Alpha::Opaque,
                )
                .on_hover_text("Average");
            })
            .add_to_row(row);
        }
        1 => {
            Setting::new("Amp Offset", |ui| {
                ui.add(
                    Slider::new(&mut trace_settings.amp_offset, -50..=50)
                        .step_by(1.0)
                        .suffix(" dB"),
                );
            })
            .add_to_row(row);
        }
        2 => {
            Setting::new("Y-Axis Max", |ui| {
                ui.add_enabled(
                    !trace_settings.autoscale_y_axis,
                    Slider::new(&mut trace_settings.y_axis_max, -130..=0)
                        .step_by(1.0)
                        .suffix(" dBm"),
                );
            })
            .add_to_row(row);
        }
        3 => {
            Setting::new("Y-Axis Min", |ui| {
                ui.add_enabled(
                    !trace_settings.autoscale_y_axis,
                    Slider::new(&mut trace_settings.y_axis_min, -130..=0)
                        .step_by(1.0)
                        .suffix(" dBm"),
                );
            })
            .add_to_row(row);
        }
        4 => {
            Setting::new("Autoscale Y-Axis", |ui| {
                ui.checkbox(&mut trace_settings.autoscale_y_axis, "");
            })
            .add_to_row(row);
        }
        5 => {
            Setting::new("Hide", |ui| {
                ui.checkbox(&mut trace_settings.hide_trace, "");
            })
            .add_to_row(row);
        }
        _ => (),
    });
}

fn show_spectrogram_settings(ui: &mut Ui, spectrogram_settings: &mut SpectrogramSettings) {
    SettingsCategory::new("Spectrogram").show(ui, 4, |row| match row.index() {
        0 => {
            Setting::new("Color Gradient", |ui| {
                SpectrogramColorGradientComboBox::show_ui(
                    ui,
                    &mut spectrogram_settings.color_gradient,
                );
                ui.add(Image::new(
                    spectrogram_settings.color_gradient.preview_image(),
                ));
            })
            .add_to_row(row);
        }
        1 => {
            Setting::new("Gradient Max", |ui| {
                ui.add(
                    Slider::new(
                        &mut spectrogram_settings.gradient_max_dbm,
                        SpectrogramSettings::MIN_AMP_DBM..=SpectrogramSettings::MAX_AMP_DBM,
                    )
                    .step_by(1.0)
                    .suffix(" dBm"),
                );
            })
            .add_to_row(row);
        }
        2 => {
            Setting::new("Gradient Min", |ui| {
                ui.add(
                    Slider::new(
                        &mut spectrogram_settings.gradient_min_dbm,
                        SpectrogramSettings::MIN_AMP_DBM..=SpectrogramSettings::MAX_AMP_DBM,
                    )
                    .step_by(1.0)
                    .suffix(" dBm"),
                );
            })
            .add_to_row(row);
        }
        3 => {
            Setting::new("Hide", |ui| {
                ui.checkbox(&mut spectrogram_settings.hide_spectrogram, "");
            })
            .add_to_row(row);
        }
        _ => (),
    });
}
