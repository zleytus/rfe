use egui::{Align, Context, Key, ScrollArea, SidePanel, TextEdit, Ui, Vec2};

use super::{InfoCategory, InfoItem, Setting, SettingsCategory};
use crate::{
    data::RfeInfo,
    settings::{FrequencyUnits, SweepSettings},
    widgets::SweepLengthComboBox,
};

pub struct RfeSettingsSidePanel {
    side_panel: SidePanel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RfeSettingsPanelResponse {
    CenterSpanChanged,
    StartStopChanged,
    SweepLenChanged,
}

impl RfeSettingsSidePanel {
    pub fn new() -> Self {
        Self {
            side_panel: SidePanel::left("rfe-settings-panel").resizable(false),
        }
    }

    pub fn show(
        self,
        ctx: &Context,
        can_change_sweep_len: bool,
        sweep_settings: &mut SweepSettings,
        rfe_info: &RfeInfo,
        units: FrequencyUnits,
    ) -> Option<RfeSettingsPanelResponse> {
        self.side_panel
            .show(ctx, |ui| {
                ScrollArea::vertical()
                    .show(ui, |ui| {
                        ui.add_space(5.0);
                        let response =
                            show_sweep_settings(ui, can_change_sweep_len, sweep_settings, units);
                        ui.add_space(10.0);
                        show_rfe_info(ui, rfe_info, units);
                        response
                    })
                    .inner
            })
            .inner
    }
}

fn show_sweep_settings(
    ui: &mut Ui,
    can_change_sweep_len: bool,
    sweep_settings: &mut SweepSettings,
    units: FrequencyUnits,
) -> Option<RfeSettingsPanelResponse> {
    let mut rfe_settings_changed = None;
    let rows = if sweep_settings.rbw.is_some() { 7 } else { 6 };
    SettingsCategory::new("Sweep").show(ui, rows, |row| match row.index() {
        0 => {
            Setting::new("Center", |ui| {
                ui.label(units.to_string());
                if ui
                    .add(
                        TextEdit::singleline(&mut sweep_settings.center_freq)
                            .min_size(Vec2::new(120.0, 20.0))
                            .horizontal_align(Align::RIGHT),
                    )
                    .lost_focus()
                    && ui.input(|i| i.key_pressed(Key::Enter))
                {
                    rfe_settings_changed = Some(RfeSettingsPanelResponse::CenterSpanChanged);
                }
            })
            .add_to_row(row);
        }
        1 => {
            Setting::new("Span", |ui| {
                ui.label(units.to_string());
                if ui
                    .add(
                        TextEdit::singleline(&mut sweep_settings.span)
                            .horizontal_align(Align::RIGHT),
                    )
                    .lost_focus()
                    && ui.input(|i| i.key_pressed(Key::Enter))
                {
                    rfe_settings_changed = Some(RfeSettingsPanelResponse::CenterSpanChanged);
                }
            })
            .add_to_row(row);
        }
        2 => {
            Setting::new("Start", |ui| {
                ui.label(units.to_string());
                if ui
                    .add(
                        TextEdit::singleline(&mut sweep_settings.start_freq)
                            .horizontal_align(Align::RIGHT),
                    )
                    .lost_focus()
                    && ui.input(|i| i.key_pressed(Key::Enter))
                {
                    rfe_settings_changed = Some(RfeSettingsPanelResponse::StartStopChanged);
                }
            })
            .add_to_row(row);
        }
        3 => {
            Setting::new("Stop", |ui| {
                ui.label(units.to_string());
                if ui
                    .add(
                        TextEdit::singleline(&mut sweep_settings.stop_freq)
                            .horizontal_align(Align::RIGHT),
                    )
                    .lost_focus()
                    && ui.input(|i| i.key_pressed(Key::Enter))
                {
                    rfe_settings_changed = Some(RfeSettingsPanelResponse::StartStopChanged);
                }
            })
            .add_to_row(row);
        }
        4 => {
            if rows == 6 {
                InfoItem::new_freq("Step Size", sweep_settings.step_size, units).add_to_row(row);
            } else {
                if let Some(rbw) = sweep_settings.rbw {
                    InfoItem::new_freq("RBW", rbw, units).add_to_row(row);
                }
            }
        }
        5 => {
            if rows == 6 {
                if can_change_sweep_len {
                    Setting::new("Length", |ui| {
                        ui.label("Points");
                        if SweepLengthComboBox::show_ui(ui, &mut sweep_settings.len)
                            .is_some_and(|r| r.changed())
                        {
                            rfe_settings_changed = Some(RfeSettingsPanelResponse::SweepLenChanged);
                        }
                    })
                    .add_to_row(row);
                } else {
                    InfoItem::new("Length", sweep_settings.len.to_string() + "  Points")
                        .add_to_row(row);
                }
            } else {
                InfoItem::new_freq("Step Size", sweep_settings.step_size, units).add_to_row(row);
            }
        }
        6 => {
            if can_change_sweep_len {
                Setting::new("Length", |ui| {
                    ui.label("Points");
                    if SweepLengthComboBox::show_ui(ui, &mut sweep_settings.len)
                        .is_some_and(|r| r.changed())
                    {
                        rfe_settings_changed = Some(RfeSettingsPanelResponse::SweepLenChanged);
                    }
                })
                .add_to_row(row);
            } else {
                InfoItem::new("Length", sweep_settings.len.to_string() + "  Points")
                    .add_to_row(row);
            }
        }
        _ => {}
    });
    return rfe_settings_changed;
}

fn show_rfe_info(ui: &mut Ui, rfe_info: &RfeInfo, units: FrequencyUnits) {
    let mut info_items = Vec::new();
    info_items.push(InfoItem::new_freq("Min Freq", rfe_info.min_freq, units));
    info_items.push(InfoItem::new_freq("Max Freq", rfe_info.max_freq, units));
    info_items.push(InfoItem::new_freq("Max Span", rfe_info.max_freq, units));
    info_items.push(InfoItem::new(
        "Active Radio",
        rfe_info.active_radio_model.to_string(),
    ));
    if let Some(inactive_radio_model) = &rfe_info.inactive_radio_model {
        info_items.push(InfoItem::new(
            "Inactive Radio",
            inactive_radio_model.to_string(),
        ));
    }
    if let Some(calc_mode) = &rfe_info.calc_mode {
        info_items.push(InfoItem::new("Calc Mode", calc_mode.to_string()));
    }
    if let Some(input_stage) = &rfe_info.input_stage {
        info_items.push(InfoItem::new("Input Stage", input_stage.to_string()));
    }
    if let Some(dsp_mode) = &rfe_info.dsp_mode {
        info_items.push(InfoItem::new("DSP Mode", dsp_mode.to_string()));
    }
    info_items.push(InfoItem::new("Port Name", rfe_info.port_name.clone()));
    info_items.push(InfoItem::new(
        "Firmware Version",
        rfe_info.firmware_version.clone(),
    ));
    if let Some(serial_number) = &rfe_info.serial_number {
        info_items.push(InfoItem::new("Serial Number", serial_number.clone()));
    }
    InfoCategory::new("RF Explorer Info").show(ui, &info_items);
}
