use std::sync::{Arc, Mutex};

use egui::{
    include_image, Button, CentralPanel, Color32, Context, CornerRadius, Image, RichText, Vec2,
};
use rfe::SpectrumAnalyzer;

#[derive(Default)]
pub struct RfeNotConnectedCentralPanel {
    central_panel: CentralPanel,
}

impl RfeNotConnectedCentralPanel {
    pub fn new() -> Self {
        Self {
            central_panel: CentralPanel::default(),
        }
    }

    pub fn show(self, ctx: &Context, rfe: &mut Option<Arc<Mutex<SpectrumAnalyzer>>>) {
        self.central_panel.show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space((ui.available_height() / 2.0) - 120.0);
                ui.add(
                    Image::new(include_image!("../../assets/usb-fill.svg"))
                        .fit_to_exact_size(Vec2::new(150.0, 200.0))
                        .tint(Color32::DARK_GRAY),
                );
                ui.label(
                    RichText::new("RF Explorer Not Connected")
                        .heading()
                        .color(Color32::WHITE)
                        .size(28.0),
                );
                ui.add_space(5.0);
                ui.style_mut().spacing.button_padding = Vec2::new(8.0, 8.0);
                if ui
                    .add(
                        Button::new(RichText::new("Try to Connect Again").size(24.0))
                            .corner_radius(CornerRadius::default().at_least(5)),
                    )
                    .clicked()
                {
                    if let Some(spectrum_analyzer) = SpectrumAnalyzer::connect() {
                        *rfe = Some(Arc::new(Mutex::new(spectrum_analyzer)));
                    }
                }
            });
        });
    }
}
