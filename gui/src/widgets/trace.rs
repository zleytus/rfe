use egui::{Ui, Vec2};
use egui_plot::{Legend, Line, Plot, PlotBounds, PlotPoint, PlotPoints, PlotResponse};
use rfe::Frequency;

use crate::{
    data::TraceData,
    settings::{FrequencyUnits, TraceSettings},
};

pub struct Trace;

impl Trace {
    pub fn show(
        ui: &mut Ui,
        trace_data: &TraceData,
        trace_settings: &TraceSettings,
        units: FrequencyUnits,
    ) -> PlotResponse<()> {
        Plot::new("trace")
            .x_axis_label(format!("Frequency ({units})"))
            .y_axis_label("Amplitude (dBm)")
            .legend(Legend::default())
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .y_axis_min_width(30.0)
            .set_margin_fraction(Vec2::new(0.005, 0.01))
            .show(ui, |plot_ui| {
                plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                    [0.0, f64::from(trace_settings.y_axis_min)],
                    [0.0, f64::from(trace_settings.y_axis_max + 1)],
                ));
                plot_ui.set_auto_bounds(egui::Vec2b {
                    x: true,
                    y: trace_settings.autoscale_y_axis,
                });
                plot_ui.line(
                    Line::new(
                        "Max",
                        sweep_to_plot_points(trace_data.max(), trace_settings.amp_offset, units),
                    )
                    .color(trace_settings.max_trace_color),
                );
                plot_ui.line(
                    Line::new(
                        "Average",
                        sweep_to_plot_points(
                            trace_data.average(),
                            trace_settings.amp_offset,
                            units,
                        ),
                    )
                    .color(trace_settings.average_trace_color),
                );
                plot_ui.line(
                    Line::new(
                        "Current",
                        sweep_to_plot_points(
                            trace_data.current(),
                            trace_settings.amp_offset,
                            units,
                        ),
                    )
                    .color(trace_settings.current_trace_color),
                );
            })
    }
}

fn sweep_to_plot_points(
    sweep: &[(Frequency, f64)],
    offset: i32,
    units: FrequencyUnits,
) -> PlotPoints<'_> {
    PlotPoints::Owned(
        sweep
            .iter()
            .map(|(freq, amp)| {
                PlotPoint::new(
                    match units {
                        FrequencyUnits::Hz => freq.as_hz_f64(),
                        FrequencyUnits::Khz => freq.as_khz_f64(),
                        FrequencyUnits::Mhz => freq.as_mhz_f64(),
                        FrequencyUnits::Ghz => freq.as_ghz_f64(),
                    },
                    *amp + f64::from(offset),
                )
            })
            .collect(),
    )
}
