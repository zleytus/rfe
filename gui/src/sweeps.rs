use rfe::Frequency;

/// The current, average, and max sweeps measured by the RF Explorer.
#[derive(Debug, Default)]
pub struct Sweeps {
    current: Vec<(Frequency, f64)>,
    average: Vec<(Frequency, f64)>,
    max: Vec<(Frequency, f64)>,
    count: usize,
    start_freq: Frequency,
    stop_freq: Frequency,
}

impl Sweeps {
    /// Updates the current, average, and max sweeps using a new sweep
    pub fn update(&mut self, sweep_amps: &[f32], start_freq: Frequency, stop_freq: Frequency) {
        // If the sweep's parameters have changed then reallocate the sweeps
        if self.current.len() != sweep_amps.len()
            || self.start_freq != start_freq
            || self.stop_freq != stop_freq
        {
            self.reallocate_sweeps(start_freq, stop_freq, sweep_amps.len());
        }

        for (i, amp_dbm) in sweep_amps.iter().enumerate() {
            self.current[i].1 = f64::from(*amp_dbm);

            // If this is the first sweep, set the average sweep to be the same as the new sweep
            // Otherwise, calculate a new average sweep using the old average sweep and the new sweep
            if self.count == 0 {
                self.average[i].1 = f64::from(*amp_dbm);
            } else {
                self.average[i].1 = (f64::from(*amp_dbm) + self.count as f64 * self.average[i].1)
                    / (self.count as f64 + 1f64);
            }

            self.max[i].1 = self.max[i].1.max(f64::from(*amp_dbm));
        }

        // Keep track of the number of times we've updated our sweeps in order to calculate averages
        self.count += 1;
    }

    fn reallocate_sweeps(&mut self, start_freq: Frequency, stop_freq: Frequency, len: usize) {
        let step_size = (stop_freq - start_freq) / u64::try_from(len - 1).unwrap_or(1);
        let mut points = Vec::new();
        for i in 0..u64::try_from(len).unwrap_or_default() {
            points.push((start_freq + step_size * i, f64::MIN));
        }
        self.current = points.clone();
        self.average = points.clone();
        self.max = points;
        self.count = 0;
        self.start_freq = start_freq;
        self.stop_freq = stop_freq;
    }

    /// Gets the current sweep.
    pub fn current(&self) -> &[(Frequency, f64)] {
        &self.current
    }

    /// Gets the average sweep.
    pub fn average(&self) -> &[(Frequency, f64)] {
        &self.average
    }

    /// Gets the max sweep.
    pub fn max(&self) -> &[(Frequency, f64)] {
        &self.max
    }

    /// Resets the average sweep.
    pub fn reset_average(&mut self) {
        self.average = self.current.clone();
        self.count = 0;
    }

    /// Resets the max sweep.
    pub fn reset_max(&mut self) {
        self.max = self.current.clone();
    }
}
