use csv::Writer;
use egui::{
    color_picker::Alpha, Align, Context, Grid, InnerResponse, Layout, Separator, SidePanel, Slider,
    Ui,
};
use rfd::FileDialog;
use rfe::Frequency;

use crate::{PlotSettings, Sweeps, Units};

pub struct PlotSettingsPanel {
    side_panel: SidePanel,
}

impl PlotSettingsPanel {
    pub fn new() -> Self {
        Self {
            side_panel: SidePanel::right("plot-settings-panel"),
        }
    }

    pub fn show(
        self,
        ctx: &Context,
        plot_settings: &mut PlotSettings,
        sweeps: &mut Sweeps,
        units: Units,
    ) -> InnerResponse<()> {
        self.side_panel.show(ctx, |ui| {
            ui.add_space(5.0);
            Grid::new("plot-settings-grid")
                .num_columns(2)
                .show(ui, |ui| {
                    axis_settings_grid(ui, plot_settings);
                    ui.add(Separator::default().grow(4.0));
                    ui.add(Separator::default().grow(4.0));
                    ui.end_row();
                    threshold_settings_grid(ui, plot_settings);
                    ui.add(Separator::default().grow(4.0));
                    ui.add(Separator::default().grow(4.0));
                    ui.end_row();
                    sweep_settings_grid(ui, plot_settings, sweeps, units);
                    ui.end_row();
                });
        })
    }
}

fn axis_settings_grid(ui: &mut Ui, plot_settings: &mut PlotSettings) {
    ui.label("Autoscale Y-Axis");
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.checkbox(&mut plot_settings.autoscale_y_axis, "");
    });
    ui.end_row();
    ui.label("Y-Axis Max");
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.add_enabled(
            !plot_settings.autoscale_y_axis,
            Slider::new(&mut plot_settings.y_axis_max, -120..=0)
                .step_by(1.0)
                .suffix(" dBm"),
        );
    });
    ui.end_row();
    ui.label("Y-Axis Min");
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.add_enabled(
            !plot_settings.autoscale_y_axis,
            Slider::new(&mut plot_settings.y_axis_min, -120..=0)
                .step_by(1.0)
                .suffix(" dBm"),
        );
    });
    ui.end_row();
    ui.label("Amp Offset");
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.add(
            Slider::new(&mut plot_settings.amp_offset, -50..=50)
                .step_by(1.0)
                .suffix(" dB"),
        );
    });
    ui.end_row();
}

fn threshold_settings_grid(ui: &mut Ui, plot_settings: &mut PlotSettings) {
    ui.label("Show Threshold");
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.checkbox(&mut plot_settings.show_threshold_line, "");
    });
    ui.end_row();
    ui.label("Threshold Value");
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.add_enabled(
            plot_settings.show_threshold_line,
            Slider::new(&mut plot_settings.threshold_line_value_dbm, -120..=0)
                .step_by(1.0)
                .suffix(" dBm"),
        );
    });
    ui.end_row();
    ui.label("Threshold Color");
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.add_enabled_ui(plot_settings.show_threshold_line, |ui| {
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut plot_settings.threshold_line_color,
                Alpha::Opaque,
            );
        });
    });
    ui.end_row();
}

fn sweep_settings_grid(
    ui: &mut Ui,
    plot_settings: &mut PlotSettings,
    sweeps: &mut Sweeps,
    units: Units,
) {
    ui.label("Current Sweep");
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        egui::color_picker::color_edit_button_srgba(
            ui,
            &mut plot_settings.current_sweep_color,
            Alpha::Opaque,
        );
        if ui
            .button("Export...")
            .on_hover_text("Export sweep as a CSV file")
            .clicked()
        {
            export_sweep(sweeps.current(), units);
        }
        ui.checkbox(&mut plot_settings.show_current_sweep, "");
    });
    ui.end_row();
    ui.label("Average Sweep");
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        egui::color_picker::color_edit_button_srgba(
            ui,
            &mut plot_settings.average_sweep_color,
            Alpha::Opaque,
        );
        if ui
            .button("Export...")
            .on_hover_text("Export sweep as a CSV file")
            .clicked()
        {
            export_sweep(sweeps.average(), units);
        }
        if ui.button("Reset").clicked() {
            sweeps.reset_average();
        }
        ui.checkbox(&mut plot_settings.show_average_sweep, "");
    });
    ui.end_row();
    ui.label("Max Sweep");
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        egui::color_picker::color_edit_button_srgba(
            ui,
            &mut plot_settings.max_sweep_color,
            Alpha::Opaque,
        );
        if ui
            .button("Export...")
            .on_hover_text("Export sweep as a CSV file")
            .clicked()
        {
            export_sweep(sweeps.max(), units);
        }
        if ui.button("Reset").clicked() {
            sweeps.reset_max();
        }
        ui.checkbox(&mut plot_settings.show_max_sweep, "");
    });
    ui.end_row();
}

fn export_sweep(sweep: &[(Frequency, f64)], units: Units) {
    let sweep = sweep.to_vec();
    std::thread::spawn(move || {
        if let Some(path) = FileDialog::new()
            .add_filter("Sweep CSV", &["csv"])
            .set_file_name("sweep")
            .save_file()
        {
            let mut writer = Writer::from_path(path).unwrap();
            for point in sweep.iter() {
                writer
                    .write_record(&[freq_to_string(point.0, units), point.1.to_string()])
                    .unwrap();
            }
            writer.flush().unwrap();
        }
    });
}

fn freq_to_string(freq: Frequency, units: Units) -> String {
    match units {
        Units::Hz => freq.as_hz().to_string(),
        Units::Khz => freq.as_khz_f64().to_string(),
        Units::Mhz => freq.as_mhz_f64().to_string(),
        Units::Ghz => freq.as_ghz_f64().to_string(),
    }
}
