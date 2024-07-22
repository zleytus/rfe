use egui::{Color32, ComboBox};

use crate::settings::Units;

#[derive(Debug, Default)]
pub struct UnitsComboBox;

impl UnitsComboBox {
    pub fn show_ui(
        self,
        ui: &mut egui::Ui,
        units: &mut Units,
        mut on_units_changed: impl FnMut(Units),
    ) {
        ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::TRANSPARENT;
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        ui.style_mut().visuals.widgets.open.weak_bg_fill = Color32::TRANSPARENT;
        ComboBox::from_id_source("units-combo-box")
            .selected_text(units.to_string())
            .width(50.0)
            .show_ui(ui, |ui| {
                for unit in [Units::Hz, Units::Khz, Units::Mhz, Units::Ghz] {
                    if ui.selectable_value(units, unit, unit.to_string()).clicked() {
                        on_units_changed(unit);
                    }
                }
            });
    }
}
