use rfe::{Frequency, SpectrumAnalyzer};

#[derive(Debug)]
pub struct Sweeps {
    current: Vec<(Frequency, f64)>,
    average: Vec<(Frequency, f64)>,
    max: Vec<(Frequency, f64)>,
    count: usize,
}

impl Sweeps {
    pub fn new(rfe: &SpectrumAnalyzer) -> Self {
        let mut points = Vec::new();
        let start_freq = rfe.start_freq();
        let step_size = rfe.step_size();
        for i in 0..u64::from(rfe.sweep_len()) {
            points.push((start_freq + step_size * i, f64::MIN));
        }
        Self {
            current: points.clone(),
            average: points.clone(),
            max: points,
            count: 0,
        }
    }

    pub fn update(&mut self, sweep_amps: &[f32]) {
        if self.current.len() != sweep_amps.len() {
            return;
        }

        for (i, amp_dbm) in sweep_amps.iter().enumerate() {
            self.current[i].1 = f64::from(*amp_dbm);
            if self.count == 0 {
                self.average[i].1 = f64::from(*amp_dbm);
            } else {
                self.average[i].1 = (f64::from(*amp_dbm) + self.count as f64 * self.average[i].1)
                    / (self.count as f64 + 1f64);
            }
            self.max[i].1 = self.max[i].1.max(f64::from(*amp_dbm));
        }

        self.count += 1;
    }

    pub fn current(&self) -> &[(Frequency, f64)] {
        &self.current
    }

    pub fn average(&self) -> &[(Frequency, f64)] {
        &self.average
    }

    pub fn max(&self) -> &[(Frequency, f64)] {
        &self.max
    }

    pub fn reset_average(&mut self) {
        self.average = self.current.clone();
        self.count = 0;
    }

    pub fn reset_max(&mut self) {
        self.max = self.current.clone();
    }
}
