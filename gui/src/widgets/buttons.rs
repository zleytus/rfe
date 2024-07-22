use egui::{Button, Color32, Response, RichText, SelectableLabel, Vec2, Widget};

#[derive(Debug, Default)]
pub struct ResumeScanningButton {}

impl Widget for ResumeScanningButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::TRANSPARENT;
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        ui.style_mut().visuals.widgets.open.weak_bg_fill = Color32::TRANSPARENT;
        Button::new(RichText::new("▶").strong().monospace())
            .min_size(Vec2::new(18.0, 18.0))
            .ui(ui)
            .on_hover_text("Resume Scanning")
    }
}

#[derive(Debug, Default)]
pub struct PauseScanningButton {}

impl Widget for PauseScanningButton {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::TRANSPARENT;
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        ui.style_mut().visuals.widgets.open.weak_bg_fill = Color32::TRANSPARENT;
        Button::new(RichText::new("⏸").strong().monospace())
            .min_size(Vec2::new(18.0, 18.0))
            .ui(ui)
            .on_hover_text("Pause Scanning")
    }
}

#[derive(Debug, Default)]
pub struct RfeSettingsToggleButton {
    selected: bool,
}

impl RfeSettingsToggleButton {
    pub fn new(selected: bool) -> Self {
        Self { selected }
    }
}

impl Widget for RfeSettingsToggleButton {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ui.style_mut().visuals.selection.bg_fill = ui.style().visuals.widgets.hovered.bg_fill;
        ui.style_mut().visuals.selection.stroke = ui.style().visuals.widgets.active.fg_stroke;
        SelectableLabel::new(self.selected, "⛭")
            .ui(ui)
            .on_hover_text("RF Explorer Settings")
    }
}

#[derive(Debug, Default)]
pub struct PlotSettingsToggleButton {
    selected: bool,
}

impl PlotSettingsToggleButton {
    pub fn new(selected: bool) -> Self {
        Self { selected }
    }
}

impl Widget for PlotSettingsToggleButton {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ui.style_mut().visuals.selection.bg_fill = ui.style().visuals.widgets.hovered.bg_fill;
        ui.style_mut().visuals.selection.stroke = ui.style().visuals.widgets.active.fg_stroke;
        SelectableLabel::new(self.selected, "🗠")
            .ui(ui)
            .on_hover_text("Plot Settings")
    }
}
