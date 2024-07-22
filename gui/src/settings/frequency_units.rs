use std::fmt::{Display, Formatter, Result};

use rfe::Frequency;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrequencyUnits {
    Hz,
    Khz,
    Mhz,
    Ghz,
}

impl FrequencyUnits {
    pub fn freq_f64(&self, freq: Frequency) -> f64 {
        match self {
            FrequencyUnits::Hz => freq.as_hz_f64(),
            FrequencyUnits::Khz => freq.as_khz_f64(),
            FrequencyUnits::Mhz => freq.as_mhz_f64(),
            FrequencyUnits::Ghz => freq.as_ghz_f64(),
        }
    }
}

impl Display for FrequencyUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Hz => write!(f, "Hz"),
            Self::Khz => write!(f, "kHz"),
            Self::Mhz => write!(f, "MHz"),
            Self::Ghz => write!(f, "GHz"),
        }
    }
}
