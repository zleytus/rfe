use egui::{Ui, Vec2, Vec2b};
use egui_plot::{Plot, PlotImage, PlotPoint, PlotResponse};

use crate::{data::SpectrogramData, settings::FrequencyUnits};

pub struct Spectrogram;

impl Spectrogram {
    pub fn show(
        ui: &mut Ui,
        spectrogram_data: &mut SpectrogramData,
        units: FrequencyUnits,
    ) -> PlotResponse<()> {
        let start = units.freq_f64(spectrogram_data.start_freq());
        let stop = units.freq_f64(spectrogram_data.stop_freq());

        let center_position =
            PlotPoint::new((start + stop) / 2.0, SpectrogramData::HEIGHT as f64 / 2.0);
        let size = Vec2::new((stop - start) as f32, SpectrogramData::HEIGHT as f32);
        let image = PlotImage::new(
            "spectrogram-image",
            spectrogram_data.texture(),
            center_position,
            size,
        );

        Plot::new("spectrogram")
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .label_formatter(|_, value| {
                format!(
                    "x = {:.1}\ny = {}",
                    value.x,
                    (value.y - SpectrogramData::HEIGHT as f64).abs() as u64
                )
            })
            .set_margin_fraction(Vec2::new(0.005, 0.01))
            .show_grid(Vec2b::FALSE)
            .x_axis_label(format!("Frequency ({units})"))
            .y_axis_label("Sweep")
            .y_axis_min_width(30.0)
            .y_axis_formatter(|grid_mark, _| {
                (grid_mark.value - SpectrogramData::HEIGHT as f64)
                    .abs()
                    .to_string()
            })
            .show(ui, |plot_ui| plot_ui.image(image))
    }
}
