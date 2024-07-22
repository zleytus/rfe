use egui::{Color32, ComboBox, Response, Ui};
use strum::IntoEnumIterator;

use crate::settings::{ColorGradient, FrequencyUnits};

#[derive(Debug, Default)]
pub struct UnitsComboBox;

impl UnitsComboBox {
    pub fn show_ui(ui: &mut Ui, units: &mut FrequencyUnits) -> Option<Response> {
        ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::TRANSPARENT;
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        ui.style_mut().visuals.widgets.open.weak_bg_fill = Color32::TRANSPARENT;
        ComboBox::from_id_salt("units-combo-box")
            .selected_text(units.to_string())
            .width(50.0)
            .show_ui(ui, |ui| {
                [
                    FrequencyUnits::Hz,
                    FrequencyUnits::Khz,
                    FrequencyUnits::Mhz,
                    FrequencyUnits::Ghz,
                ]
                .iter()
                .map(|unit| ui.selectable_value(units, *unit, unit.to_string()))
                .reduce(|acc, e| acc | e)
                .unwrap()
            })
            .inner
    }
}

#[derive(Debug, Default)]
pub struct SpectrogramColorGradientComboBox;

impl SpectrogramColorGradientComboBox {
    pub fn show_ui(ui: &mut Ui, color_gradient: &mut ColorGradient) -> Option<Response> {
        ComboBox::from_id_salt("colors-combo-box")
            .selected_text(color_gradient.to_string())
            .show_ui(ui, |ui| {
                ColorGradient::iter()
                    .map(|gradient| {
                        ui.selectable_value(color_gradient, gradient, gradient.to_string())
                    })
                    .reduce(|acc, e| acc | e)
                    .unwrap()
            })
            .inner
    }
}

#[derive(Debug, Default)]
pub struct SweepLengthComboBox;

impl SweepLengthComboBox {
    pub fn show_ui(ui: &mut Ui, sweep_len: &mut u16) -> Option<Response> {
        ComboBox::from_id_salt("sweep-length-combo-box")
            .selected_text(sweep_len.to_string())
            .width(30.0)
            .show_ui(ui, |ui| {
                [112, 240, 512, 1024]
                    .iter()
                    .map(|len| ui.selectable_value(sweep_len, *len, len.to_string()))
                    .reduce(|acc, e| acc | e)
                    .unwrap()
            })
            .inner
    }
}
