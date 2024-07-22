use rfe::Frequency;

/// The current, average, and max traces measured by the RF Explorer.
#[derive(Debug, Clone)]
pub struct TraceData {
    current: Vec<(Frequency, f64)>,
    average: Vec<(Frequency, f64)>,
    max: Vec<(Frequency, f64)>,
    is_first_trace: bool,
    start_freq: Frequency,
    stop_freq: Frequency,
    step_size: Frequency,
}

impl TraceData {
    const AVERAGE_ITERATIONS: f64 = 5.0;

    /// Updates the current, average, and max traces using a new sweep.
    pub fn update(&mut self, amps_dbm: &[f32], start_freq: Frequency, stop_freq: Frequency) {
        // If the sweep's parameters have changed then reset the data
        if self.current.len() != amps_dbm.len()
            || self.start_freq != start_freq
            || self.stop_freq != stop_freq
        {
            self.reset_data(start_freq, stop_freq, amps_dbm.len());
        }

        for (i, amp_dbm) in amps_dbm.iter().enumerate() {
            self.current[i].1 = f64::from(*amp_dbm);

            // If this is the first trace, set the average trace to be the same as the new trace
            // Otherwise, calculate a new average trace using the old average trace and the new trace
            if self.is_first_trace {
                self.average[i].1 = f64::from(*amp_dbm);
            } else {
                self.average[i].1 -= self.average[i].1 / Self::AVERAGE_ITERATIONS;
                self.average[i].1 += f64::from(*amp_dbm) / Self::AVERAGE_ITERATIONS;
            }

            self.max[i].1 = self.max[i].1.max(f64::from(*amp_dbm));
        }

        self.is_first_trace = false;
    }

    fn reset_data(&mut self, start_freq: Frequency, stop_freq: Frequency, len: usize) {
        let step_size = (stop_freq - start_freq) / u64::try_from(len - 1).unwrap_or(1);
        let mut points = Vec::new();
        for i in 0..u64::try_from(len).unwrap_or_default() {
            points.push((start_freq + step_size * i, f64::MIN));
        }
        self.current = points.clone();
        self.average = points.clone();
        self.max = points;
        self.is_first_trace = true;
        self.start_freq = start_freq;
        self.stop_freq = stop_freq;
        self.step_size = step_size;
    }

    /// Gets the current trace.
    pub fn current(&self) -> &[(Frequency, f64)] {
        &self.current
    }

    /// Gets the average trace.
    pub fn average(&self) -> &[(Frequency, f64)] {
        &self.average
    }

    /// Gets the max trace.
    pub fn max(&self) -> &[(Frequency, f64)] {
        &self.max
    }
}

impl Default for TraceData {
    fn default() -> Self {
        Self {
            current: Vec::default(),
            average: Vec::default(),
            max: Vec::default(),
            is_first_trace: true,
            start_freq: Frequency::default(),
            stop_freq: Frequency::default(),
            step_size: Frequency::default(),
        }
    }
}
