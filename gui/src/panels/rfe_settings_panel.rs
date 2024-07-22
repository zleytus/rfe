use std::{num::ParseFloatError, str::FromStr};

use egui::{
    text_edit::TextEditOutput, Align, ComboBox, Context, Grid, InnerResponse, Key, Label, Layout,
    Separator, SidePanel, TextEdit, Ui, WidgetText,
};
use rfe::{Frequency, SpectrumAnalyzer};

use crate::settings::{RfeInfo, RfeSettings, SweepSettings, Units};

pub struct RfeSettingsPanel {
    side_panel: SidePanel,
}

impl RfeSettingsPanel {
    pub fn new() -> Self {
        Self {
            side_panel: SidePanel::left("rfe-settings-panel"),
        }
    }

    pub fn show(
        self,
        ctx: &Context,
        rfe: Option<&SpectrumAnalyzer>,
        rfe_settings: &mut RfeSettings,
        units: Units,
    ) -> InnerResponse<()> {
        self.side_panel.show(ctx, |ui| {
            ui.add_space(5.0);
            Grid::new("rfe-settings-grid")
                .num_columns(2)
                .show(ui, |ui| {
                    sweep_settings_grid(ui, rfe, &mut rfe_settings.sweep_settings, units);
                    ui.add(Separator::default().grow(4.0));
                    ui.add(Separator::default().grow(4.0));
                    ui.end_row();
                    rfe_info_grid(ui, &rfe_settings.rfe_info, units);
                    ui.end_row();
                });
        })
    }
}

fn sweep_settings_grid(
    ui: &mut Ui,
    rfe: Option<&SpectrumAnalyzer>,
    sweep_settings: &mut SweepSettings,
    units: Units,
) {
    let center_freq = freq_input_item(ui, "Center", &mut sweep_settings.center_freq, units);
    if center_freq.response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
        if let Some(ref rfe) = rfe {
            _ = set_center_span(
                rfe,
                &sweep_settings.center_freq,
                &sweep_settings.span,
                units,
            );
        }
    }
    let span = freq_input_item(ui, "Span", &mut sweep_settings.span, units);
    if span.response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
        if let Some(ref rfe) = rfe {
            _ = set_center_span(
                rfe,
                &sweep_settings.center_freq,
                &sweep_settings.span,
                units,
            );
        }
    }
    let start_freq = freq_input_item(ui, "Start", &mut sweep_settings.start_freq, units);
    if start_freq.response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
        if let Some(ref rfe) = rfe {
            _ = set_start_stop(
                rfe,
                &sweep_settings.start_freq,
                &sweep_settings.stop_freq,
                units,
            );
        }
    }
    let stop_freq = freq_input_item(ui, "Stop", &mut sweep_settings.stop_freq, units);
    if stop_freq.response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
        if let Some(ref rfe) = rfe {
            _ = set_start_stop(
                rfe,
                &sweep_settings.start_freq,
                &sweep_settings.stop_freq,
                units,
            );
        }
    }
    if let Some(rbw) = sweep_settings.rbw {
        freq_info_item(ui, "RBW", rbw, units);
    }
    freq_info_item(ui, "Step Size", sweep_settings.step_size, units);
    match rfe {
        Some(rfe) if rfe.active_radio_model().is_plus_model() => {
            ui.label("Length");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_sized([35.0, 20.0], Label::new("Points"));
                let i = ComboBox::from_id_source("length-combo-box")
                    .selected_text(sweep_settings.len.to_string())
                    .width(30.0)
                    .show_ui(ui, |ui| {
                        for len in [
                            112, 240, 512, 1024, 2048, 3072, 4096, 5120, 6144, 7168, 8192, 9216,
                        ] {
                            if ui
                                .selectable_value(&mut sweep_settings.len, len, len.to_string())
                                .clicked()
                            {
                                _ = set_center_span_sweep_len(
                                    rfe,
                                    &sweep_settings.center_freq,
                                    &sweep_settings.span,
                                    units,
                                    len,
                                );
                            }
                        }
                    });
                if i.response.changed() {
                    println!("Combo box changed");
                }
            });
            ui.end_row();
        }
        _ => {
            sweep_info_item(ui, "Length", &sweep_settings.len.to_string(), "Points");
        }
    }
}

fn rfe_info_grid(ui: &mut Ui, rfe_info: &RfeInfo, units: Units) {
    rfe_freq_info_item(ui, "Min Freq", rfe_info.min_freq, units);
    rfe_freq_info_item(ui, "Max Freq", rfe_info.max_freq, units);
    rfe_freq_info_item(ui, "Max Span", rfe_info.max_span, units);
    rfe_info_item(ui, "Active Radio", &rfe_info.active_radio_model);
    if let Some(inactive_radio_model) = &rfe_info.inactive_radio_model {
        rfe_info_item(ui, "Inactive Radio", inactive_radio_model);
    }
    if let Some(calc_mode) = &rfe_info.calc_mode {
        rfe_info_item(ui, "Calc Mode", calc_mode);
    }
    if let Some(input_stage) = &rfe_info.input_stage {
        rfe_info_item(ui, "Input Stage", input_stage);
    }
    if let Some(dsp_mode) = &rfe_info.dsp_mode {
        rfe_info_item(ui, "DSP Mode", dsp_mode);
    }
    rfe_info_item(ui, "Port Name", &rfe_info.port_name);
    // if let Ok(baud_rate) = rfe.baud_rate() {
    //     ui.label("Baud Rate");
    //     ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
    //         ui.label(format!("{} bps", baud_rate));
    //     });
    //     ui.end_row();
    // }
    rfe_info_item(ui, "Firmware Version", &rfe_info.firmware_version);
    if let Some(serial_number) = &rfe_info.serial_number {
        rfe_info_item(ui, "Serial Number", serial_number);
    }
}

fn freq_input_item(
    ui: &mut egui::Ui,
    label: &str,
    freq: &mut String,
    units: Units,
) -> TextEditOutput {
    ui.label(label);
    let text_edit_output = ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.horizontal(|ui| {
            ui.add_sized([35.0, 20.0], Label::new(units.to_string()));
            TextEdit::singleline(freq)
                .horizontal_align(Align::Max)
                .show(ui)
        })
    });
    ui.end_row();
    return text_edit_output.inner.inner;
}

fn freq_info_item(ui: &mut Ui, label: &str, freq: Frequency, units: Units) {
    sweep_info_item(
        ui,
        label,
        &match units {
            Units::Hz => freq.as_hz().to_string(),
            Units::Khz => freq.as_khz_f64().to_string(),
            Units::Mhz => freq.as_mhz_f64().to_string(),
            Units::Ghz => freq.as_ghz_f64().to_string(),
        },
        units.to_string(),
    );
}

fn sweep_info_item(ui: &mut Ui, label: &str, mut value: &str, units: impl Into<WidgetText>) {
    ui.label(label);
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.add_sized([35.0, 20.0], Label::new(units));
        ui.add(TextEdit::singleline(&mut value).horizontal_align(Align::Max))
    });
    ui.end_row();
}

fn rfe_freq_info_item(ui: &mut Ui, label: impl Into<WidgetText>, freq: Frequency, units: Units) {
    let value = match units {
        Units::Hz => freq.as_hz().to_string() + " Hz",
        Units::Khz => freq.as_khz_f64().to_string() + " kHz",
        Units::Mhz => freq.as_mhz_f64().to_string() + " MHz",
        Units::Ghz => freq.as_ghz_f64().to_string() + " GHz",
    };
    rfe_info_item(ui, label, value);
}

fn rfe_info_item(ui: &mut Ui, label: impl Into<WidgetText>, value: impl Into<WidgetText>) {
    ui.label(label);
    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
        ui.label(value);
    });
    ui.end_row();
}

fn set_center_span(
    rfe: &SpectrumAnalyzer,
    center_freq: &str,
    span: &str,
    units: Units,
) -> Result<(), ()> {
    let (center, span) = str_to_freq(center_freq, span, units).map_err(|_| ())?;
    rfe.set_center_span(center, span).map_err(|_| ())
}

fn set_center_span_sweep_len(
    rfe: &SpectrumAnalyzer,
    center_freq: &str,
    span: &str,
    units: Units,
    sweep_len: u16,
) -> Result<(), ()> {
    let (center, span) = str_to_freq(center_freq, span, units).map_err(|_| ())?;
    rfe.set_center_span_sweep_len(center, span, sweep_len)
        .map_err(|_| ())
}

fn set_start_stop(
    rfe: &SpectrumAnalyzer,
    start_freq: &str,
    stop_freq: &str,
    units: Units,
) -> Result<(), ()> {
    let (start, stop) = str_to_freq(start_freq, stop_freq, units).map_err(|_| ())?;
    rfe.set_start_stop(start, stop).map_err(|_| ())
}

fn str_to_freq(
    str1: &str,
    str2: &str,
    units: Units,
) -> Result<(Frequency, Frequency), ParseFloatError> {
    Ok(match units {
        Units::Hz => (
            Frequency::from_hz(f64::from_str(str1)? as u64),
            Frequency::from_hz(f64::from_str(str2)? as u64),
        ),
        Units::Khz => (
            Frequency::from_khz_f64(f64::from_str(str1)?),
            Frequency::from_khz_f64(f64::from_str(str2)?),
        ),
        Units::Mhz => (
            Frequency::from_mhz_f64(f64::from_str(str1)?),
            Frequency::from_mhz_f64(f64::from_str(str2)?),
        ),
        Units::Ghz => (
            Frequency::from_ghz_f64(f64::from_str(str1)?),
            Frequency::from_ghz_f64(f64::from_str(str2)?),
        ),
    })
}
