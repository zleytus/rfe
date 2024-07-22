use egui_plot::{PlotBounds, PlotPoint, PlotPoints, PlotResponse};
use rfe::Frequency;

use crate::{PlotSettings, Sweeps, Units};

pub struct Plot;

impl Plot {
    pub fn show(
        ui: &mut egui::Ui,
        sweeps: &Sweeps,
        plot_settings: &PlotSettings,
        units: Units,
    ) -> PlotResponse<()> {
        egui_plot::Plot::new("plot")
            .x_axis_label(format!("Frequency ({units})"))
            .y_axis_label("Amplitude (dBm)")
            .show(ui, |plot_ui| {
                plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                    [0.0, f64::from(plot_settings.y_axis_min)],
                    [0.0, f64::from(plot_settings.y_axis_max)],
                ));
                plot_ui.set_auto_bounds(egui::Vec2b {
                    x: true,
                    y: plot_settings.autoscale_y_axis,
                });
                if plot_settings.show_max_sweep {
                    plot_ui.line(
                        egui_plot::Line::new(sweep_to_plot_points(
                            sweeps.max(),
                            plot_settings.amp_offset,
                            units,
                        ))
                        .name("Max")
                        .color(plot_settings.max_sweep_color),
                    );
                }
                if plot_settings.show_average_sweep {
                    plot_ui.line(
                        egui_plot::Line::new(sweep_to_plot_points(
                            sweeps.average(),
                            plot_settings.amp_offset,
                            units,
                        ))
                        .name("Average")
                        .color(plot_settings.average_sweep_color),
                    );
                }
                if plot_settings.show_current_sweep {
                    plot_ui.line(
                        egui_plot::Line::new(sweep_to_plot_points(
                            sweeps.current(),
                            plot_settings.amp_offset,
                            units,
                        ))
                        .name("Current")
                        .color(plot_settings.current_sweep_color),
                    );
                }
                if plot_settings.show_threshold_line {
                    plot_ui.hline(
                        egui_plot::HLine::new(plot_settings.threshold_line_value_dbm)
                            .color(plot_settings.threshold_line_color),
                    );
                }
            })
    }
}

fn sweep_to_plot_points(sweep: &[(Frequency, f64)], offset: i32, units: Units) -> PlotPoints {
    PlotPoints::Owned(
        sweep
            .iter()
            .map(|(freq, amp)| {
                PlotPoint::new(
                    match units {
                        Units::Hz => freq.as_hz() as f64,
                        Units::Khz => freq.as_khz_f64(),
                        Units::Mhz => freq.as_mhz_f64(),
                        Units::Ghz => freq.as_ghz_f64(),
                    },
                    *amp + f64::from(offset),
                )
            })
            .collect(),
    )
}
