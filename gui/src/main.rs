#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() -> eframe::Result {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([500.0, 250.0])
            .with_inner_size([1500.0, 750.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
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
