use egui::Color32;

use super::ColorGradient;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SpectrogramSettings {
    pub color_gradient: ColorGradient,
    pub gradient_min_dbm: i16,
    pub gradient_max_dbm: i16,
    pub hide_spectrogram: bool,
}

impl SpectrogramSettings {
    pub const MIN_AMP_DBM: i16 = -130;
    pub const MAX_AMP_DBM: i16 = 0;

    /// Converts an amplitude to a color in the color gradient.
    pub fn amp_to_color(&self, amp: f64) -> Color32 {
        let color = self.color_gradient.gradient().eval_continuous(
            (amp - self.gradient_min_dbm as f64)
                / (self.gradient_max_dbm as f64 - self.gradient_min_dbm as f64).abs(),
        );
        Color32::from_rgb(color.r, color.g, color.b)
    }
}

impl Default for SpectrogramSettings {
    fn default() -> Self {
        Self {
            color_gradient: ColorGradient::Turbo,
            gradient_min_dbm: -105,
            gradient_max_dbm: -40,
            hide_spectrogram: false,
        }
    }
}
