#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(
            // NOTE: Adding an icon is optional
            eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                .unwrap(),
        ),
        ..Default::default()
    };
    eframe::run_native(
        "RF Explorer",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(rfe_gui::App::new(
                cc,
                rfe::SpectrumAnalyzer::connect(),
            )))
        }),
    )
}
