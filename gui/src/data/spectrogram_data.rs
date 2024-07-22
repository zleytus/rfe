use std::sync::Arc;

use egui::{Color32, ColorImage, Context, ImageData, TextureHandle, TextureOptions};
use rfe::Frequency;
use ringbuffer::{AllocRingBuffer, RingBuffer};

use crate::settings::SpectrogramSettings;

/// The image data and sweep history needed to display a spectrogram.
pub struct SpectrogramData {
    texture: TextureHandle,
    image: ColorImage,
    sweep_history: AllocRingBuffer<Vec<f32>>,
    start_freq: Frequency,
    stop_freq: Frequency,
}

impl SpectrogramData {
    pub const HEIGHT: usize = 100;

    pub fn new(ctx: &Context) -> Self {
        let image = ColorImage::new([0, 0], Color32::TRANSPARENT);
        Self {
            texture: ctx.load_texture("spectrogram", image.clone(), TextureOptions::default()),
            image,
            sweep_history: AllocRingBuffer::new(Self::HEIGHT),
            start_freq: Frequency::default(),
            stop_freq: Frequency::default(),
        }
    }

    /// Updates the spectrogram data by adding a new sweep.
    pub fn update(
        &mut self,
        sweep_amps: &[f32],
        start_freq: Frequency,
        stop_freq: Frequency,
        spectrogram_settings: &SpectrogramSettings,
    ) {
        // If the sweep's parameters have changed then reset the data
        if self.image.width() != sweep_amps.len()
            || self.start_freq != start_freq
            || self.stop_freq != stop_freq
        {
            self.reset_data(start_freq, stop_freq, sweep_amps.len());
        }

        // Shift each row in the image down 1
        let image_width = self.image.width();
        for row in (1..self.image.height()).rev() {
            for col in 0..image_width {
                self.image.pixels[image_width * row + col] =
                    self.image.pixels[image_width * (row - 1) + col];
            }
        }

        // Update the first row of the image with colors from the latest sweep
        for (i, amp) in sweep_amps.iter().map(|amp| f64::from(*amp)).enumerate() {
            self.image.pixels[i] = spectrogram_settings.amp_to_color(amp);
        }

        // Save the sweep in case we need to recreate the image later
        self.sweep_history.push(sweep_amps.to_vec());

        // Set the updated image to the spectrogram texture
        self.texture.set(
            ImageData::Color(Arc::new(self.image.clone())),
            TextureOptions::default(),
        );
    }

    fn reset_data(&mut self, start_freq: Frequency, stop_freq: Frequency, sweep_len: usize) {
        self.image = ColorImage::new([sweep_len, Self::HEIGHT], Color32::TRANSPARENT);
        self.sweep_history.clear();
        self.start_freq = start_freq;
        self.stop_freq = stop_freq;
    }

    /// Gets the start frequency of the spectrogram data.
    pub fn start_freq(&self) -> Frequency {
        self.start_freq
    }

    /// Gets the stop frequency of the spectrogram data.
    pub fn stop_freq(&self) -> Frequency {
        self.stop_freq
    }

    /// Gets a reference to the spectrogram's texture.
    pub fn texture(&self) -> &TextureHandle {
        &self.texture
    }

    /// Recreates the spectrogram's image using a saved history of sweeps.
    pub fn recreate_image(&mut self, spectrogram_settings: &SpectrogramSettings) {
        // Recalculate the color of each pixel in the image using the sweep history
        let image_width = self.image.width();
        for (row, sweep) in self.sweep_history.iter().enumerate() {
            for (i, amp) in sweep.iter().map(|amp| f64::from(*amp)).enumerate() {
                self.image.pixels[row * image_width + i] = spectrogram_settings.amp_to_color(amp);
            }
        }

        // Set the updated image to the spectrogram texture
        self.texture.set(
            ImageData::Color(Arc::new(self.image.clone())),
            TextureOptions::default(),
        );
    }
}
