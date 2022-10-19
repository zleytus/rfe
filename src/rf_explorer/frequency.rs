use std::ops::{Add, Div, Mul, Sub};

use uom::si::{
    f32, f64,
    frequency::{gigahertz, hertz, kilohertz, megahertz},
    u64,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Frequency {
    freq: u64::Frequency,
}

impl Frequency {
    pub fn from_hz(hz: u64) -> Frequency {
        Frequency {
            freq: u64::Frequency::new::<hertz>(hz),
        }
    }

    pub fn from_khz(khz: u64) -> Frequency {
        Frequency {
            freq: u64::Frequency::new::<kilohertz>(khz),
        }
    }

    pub fn from_khz_f32(khz: f32) -> Frequency {
        if khz.is_sign_negative() || (u64::MAX as f32) < khz {
            return Frequency::default();
        }

        Frequency {
            freq: u64::Frequency::new::<hertz>(
                f32::Frequency::new::<kilohertz>(khz).get::<hertz>() as u64,
            ),
        }
    }

    pub fn from_khz_f64(khz: f64) -> Frequency {
        if khz.is_sign_negative() || (u64::MAX as f64) < khz {
            return Frequency::default();
        }

        Frequency {
            freq: u64::Frequency::new::<hertz>(
                f64::Frequency::new::<kilohertz>(khz).get::<hertz>() as u64,
            ),
        }
    }

    pub fn from_mhz(mhz: u64) -> Frequency {
        Frequency {
            freq: u64::Frequency::new::<megahertz>(mhz),
        }
    }

    pub fn from_mhz_f32(mhz: f32) -> Frequency {
        if mhz.is_sign_negative() || (u64::MAX as f32) < mhz {
            return Frequency::default();
        }

        Frequency {
            freq: u64::Frequency::new::<hertz>(
                f32::Frequency::new::<megahertz>(mhz).get::<hertz>() as u64,
            ),
        }
    }

    pub fn from_mhz_f64(mhz: f64) -> Frequency {
        if mhz.is_sign_negative() || (u64::MAX as f64) < mhz {
            return Frequency::default();
        }

        Frequency {
            freq: u64::Frequency::new::<hertz>(
                f64::Frequency::new::<megahertz>(mhz).get::<hertz>() as u64,
            ),
        }
    }

    pub fn from_ghz(ghz: u64) -> Frequency {
        Frequency {
            freq: u64::Frequency::new::<gigahertz>(ghz),
        }
    }

    pub fn from_ghz_f32(ghz: f32) -> Frequency {
        if ghz.is_sign_negative() || (u64::MAX as f32) < ghz {
            return Frequency::default();
        }

        Frequency {
            freq: u64::Frequency::new::<hertz>(
                f32::Frequency::new::<gigahertz>(ghz).get::<hertz>() as u64,
            ),
        }
    }

    pub fn from_ghz_f64(ghz: f64) -> Frequency {
        if ghz.is_sign_negative() || (u64::MAX as f64) < ghz {
            return Frequency::default();
        }

        Frequency {
            freq: u64::Frequency::new::<hertz>(
                f64::Frequency::new::<gigahertz>(ghz).get::<hertz>() as u64,
            ),
        }
    }

    pub fn as_hz(&self) -> u64 {
        self.freq.get::<hertz>()
    }

    pub fn as_khz(&self) -> u64 {
        self.freq.get::<kilohertz>()
    }

    pub fn as_khz_f32(&self) -> f32 {
        f32::Frequency::new::<hertz>(self.freq.get::<hertz>() as f32).get::<kilohertz>()
    }

    pub fn as_khz_f64(&self) -> f64 {
        f64::Frequency::new::<hertz>(self.freq.get::<hertz>() as f64).get::<kilohertz>()
    }

    pub fn as_mhz(&self) -> u64 {
        self.freq.get::<megahertz>()
    }

    pub fn as_mhz_f32(&self) -> f32 {
        f32::Frequency::new::<hertz>(self.freq.get::<hertz>() as f32).get::<megahertz>()
    }

    pub fn as_mhz_f64(&self) -> f64 {
        f64::Frequency::new::<hertz>(self.freq.get::<hertz>() as f64).get::<megahertz>()
    }

    pub fn as_ghz(&self) -> u64 {
        self.freq.get::<gigahertz>()
    }

    pub fn as_ghz_f32(&self) -> f32 {
        f32::Frequency::new::<hertz>(self.freq.get::<hertz>() as f32).get::<gigahertz>()
    }

    pub fn as_ghz_f64(&self) -> f64 {
        f64::Frequency::new::<hertz>(self.freq.get::<hertz>() as f64).get::<gigahertz>()
    }
}

impl Add for Frequency {
    type Output = Frequency;

    fn add(self, rhs: Frequency) -> Self::Output {
        Frequency {
            freq: self.freq + rhs.freq,
        }
    }
}

impl Sub for Frequency {
    type Output = Frequency;

    fn sub(self, rhs: Frequency) -> Self::Output {
        if self < rhs {
            panic!("Cannot subtract a larger frequency from a smaller frequency");
        }

        Frequency {
            freq: self.freq - rhs.freq,
        }
    }
}

impl Mul<u64> for Frequency {
    type Output = Frequency;

    fn mul(self, rhs: u64) -> Self::Output {
        Frequency {
            freq: self.freq * rhs,
        }
    }
}

impl Div<u64> for Frequency {
    type Output = Frequency;

    fn div(self, rhs: u64) -> Self::Output {
        if rhs == 0 {
            panic!("Cannot divide a frequency by zero.");
        }

        Frequency {
            freq: self.freq / rhs,
        }
    }
}

impl From<u64> for Frequency {
    fn from(freq_hz: u64) -> Self {
        Frequency::from_hz(freq_hz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequency_to_hz() {
        let frequency = Frequency::from_hz(1_000_000_000);
        assert_eq!(frequency.as_hz(), 1_000_000_000);

        let frequency = Frequency::from_khz(1_000_000);
        assert_eq!(frequency.as_hz(), 1_000_000_000);

        let frequency = Frequency::from_khz_f32(1_000_000.);
        assert_eq!(frequency.as_hz(), 1_000_000_000);

        let frequency = Frequency::from_khz_f64(1_000_000.);
        assert_eq!(frequency.as_hz(), 1_000_000_000);

        let frequency = Frequency::from_mhz(1_000);
        assert_eq!(frequency.as_hz(), 1_000_000_000);

        let frequency = Frequency::from_mhz_f32(1_000.);
        assert_eq!(frequency.as_hz(), 1_000_000_000);

        let frequency = Frequency::from_mhz_f64(1_000.);
        assert_eq!(frequency.as_hz(), 1_000_000_000);

        let frequency = Frequency::from_ghz(1);
        assert_eq!(frequency.as_hz(), 1_000_000_000);

        let frequency = Frequency::from_ghz_f32(1.);
        assert_eq!(frequency.as_hz(), 1_000_000_000);

        let frequency = Frequency::from_ghz_f64(1.);
        assert_eq!(frequency.as_hz(), 1_000_000_000);
    }

    #[test]
    fn frequency_to_khz() {
        let frequency = Frequency::from_hz(1_000_000_000);
        assert_eq!(frequency.as_khz(), 1_000_000);
        assert_eq!(frequency.as_khz_f32(), 1_000_000.);
        assert_eq!(frequency.as_khz_f64(), 1_000_000.);

        let frequency = Frequency::from_khz(1_000_000);
        assert_eq!(frequency.as_khz(), 1_000_000);
        assert_eq!(frequency.as_khz_f32(), 1_000_000.);
        assert_eq!(frequency.as_khz_f64(), 1_000_000.);

        let frequency = Frequency::from_khz_f32(1_000_000.);
        assert_eq!(frequency.as_khz(), 1_000_000);
        assert_eq!(frequency.as_khz_f32(), 1_000_000.);
        assert_eq!(frequency.as_khz_f64(), 1_000_000.);

        let frequency = Frequency::from_khz_f64(1_000_000.);
        assert_eq!(frequency.as_khz(), 1_000_000);
        assert_eq!(frequency.as_khz_f32(), 1_000_000.);
        assert_eq!(frequency.as_khz_f64(), 1_000_000.);

        let frequency = Frequency::from_mhz(1_000);
        assert_eq!(frequency.as_khz(), 1_000_000);
        assert_eq!(frequency.as_khz_f32(), 1_000_000.);
        assert_eq!(frequency.as_khz_f64(), 1_000_000.);

        let frequency = Frequency::from_mhz_f32(1_000.);
        assert_eq!(frequency.as_khz(), 1_000_000);
        assert_eq!(frequency.as_khz_f32(), 1_000_000.);
        assert_eq!(frequency.as_khz_f64(), 1_000_000.);

        let frequency = Frequency::from_mhz_f64(1_000.);
        assert_eq!(frequency.as_khz(), 1_000_000);
        assert_eq!(frequency.as_khz_f32(), 1_000_000.);
        assert_eq!(frequency.as_khz_f64(), 1_000_000.);

        let frequency = Frequency::from_ghz(1);
        assert_eq!(frequency.as_khz(), 1_000_000);
        assert_eq!(frequency.as_khz_f32(), 1_000_000.);
        assert_eq!(frequency.as_khz_f64(), 1_000_000.);

        let frequency = Frequency::from_ghz_f32(1.);
        assert_eq!(frequency.as_khz(), 1_000_000);
        assert_eq!(frequency.as_khz_f32(), 1_000_000.);
        assert_eq!(frequency.as_khz_f64(), 1_000_000.);

        let frequency = Frequency::from_ghz_f64(1.);
        assert_eq!(frequency.as_khz(), 1_000_000);
        assert_eq!(frequency.as_khz_f32(), 1_000_000.);
        assert_eq!(frequency.as_khz_f64(), 1_000_000.);
    }

    #[test]
    fn frequency_to_mhz() {
        let frequency = Frequency::from_hz(1_000_000_000);
        assert_eq!(frequency.as_mhz(), 1_000);
        assert_eq!(frequency.as_mhz_f32(), 1_000.);
        assert_eq!(frequency.as_mhz_f64(), 1_000.);

        let frequency = Frequency::from_khz(1_000_000);
        assert_eq!(frequency.as_mhz(), 1_000);
        assert_eq!(frequency.as_mhz_f32(), 1_000.);
        assert_eq!(frequency.as_mhz_f64(), 1_000.);

        let frequency = Frequency::from_khz_f32(1_000_000.);
        assert_eq!(frequency.as_mhz(), 1_000);
        assert_eq!(frequency.as_mhz_f32(), 1_000.);
        assert_eq!(frequency.as_mhz_f64(), 1_000.);

        let frequency = Frequency::from_khz_f64(1_000_000.);
        assert_eq!(frequency.as_mhz(), 1_000);
        assert_eq!(frequency.as_mhz_f32(), 1_000.);
        assert_eq!(frequency.as_mhz_f64(), 1_000.);

        let frequency = Frequency::from_mhz(1_000);
        assert_eq!(frequency.as_mhz(), 1_000);
        assert_eq!(frequency.as_mhz_f32(), 1_000.);
        assert_eq!(frequency.as_mhz_f64(), 1_000.);

        let frequency = Frequency::from_mhz_f32(1_000.);
        assert_eq!(frequency.as_mhz(), 1_000);
        assert_eq!(frequency.as_mhz_f32(), 1_000.);
        assert_eq!(frequency.as_mhz_f64(), 1_000.);

        let frequency = Frequency::from_mhz_f64(1_000.);
        assert_eq!(frequency.as_mhz(), 1_000);
        assert_eq!(frequency.as_mhz_f32(), 1_000.);
        assert_eq!(frequency.as_mhz_f64(), 1_000.);

        let frequency = Frequency::from_ghz(1);
        assert_eq!(frequency.as_mhz(), 1_000);
        assert_eq!(frequency.as_mhz_f32(), 1_000.);
        assert_eq!(frequency.as_mhz_f64(), 1_000.);

        let frequency = Frequency::from_ghz_f32(1.);
        assert_eq!(frequency.as_mhz(), 1_000);
        assert_eq!(frequency.as_mhz_f32(), 1_000.);
        assert_eq!(frequency.as_mhz_f64(), 1_000.);

        let frequency = Frequency::from_ghz_f64(1.);
        assert_eq!(frequency.as_mhz(), 1_000);
        assert_eq!(frequency.as_mhz_f32(), 1_000.);
        assert_eq!(frequency.as_mhz_f64(), 1_000.);
    }

    #[test]
    fn frequency_to_ghz() {
        let frequency = Frequency::from_hz(1_000_000_000);
        assert_eq!(frequency.as_ghz(), 1);
        assert_eq!(frequency.as_ghz_f32(), 1.);
        assert_eq!(frequency.as_ghz_f64(), 1.);

        let frequency = Frequency::from_khz(1_000_000);
        assert_eq!(frequency.as_ghz(), 1);
        assert_eq!(frequency.as_ghz_f32(), 1.);
        assert_eq!(frequency.as_ghz_f64(), 1.);

        let frequency = Frequency::from_khz_f32(1_000_000.);
        assert_eq!(frequency.as_ghz(), 1);
        assert_eq!(frequency.as_ghz_f32(), 1.);
        assert_eq!(frequency.as_ghz_f64(), 1.);

        let frequency = Frequency::from_khz_f64(1_000_000.);
        assert_eq!(frequency.as_ghz(), 1);
        assert_eq!(frequency.as_ghz_f32(), 1.);
        assert_eq!(frequency.as_ghz_f64(), 1.);

        let frequency = Frequency::from_mhz(1_000);
        assert_eq!(frequency.as_ghz(), 1);
        assert_eq!(frequency.as_ghz_f32(), 1.);
        assert_eq!(frequency.as_ghz_f64(), 1.);

        let frequency = Frequency::from_mhz_f32(1_000.);
        assert_eq!(frequency.as_ghz(), 1);
        assert_eq!(frequency.as_ghz_f32(), 1.);
        assert_eq!(frequency.as_ghz_f64(), 1.);

        let frequency = Frequency::from_mhz_f64(1_000.);
        assert_eq!(frequency.as_ghz(), 1);
        assert_eq!(frequency.as_ghz_f32(), 1.);
        assert_eq!(frequency.as_ghz_f64(), 1.);

        let frequency = Frequency::from_ghz(1);
        assert_eq!(frequency.as_ghz(), 1);
        assert_eq!(frequency.as_ghz_f32(), 1.);
        assert_eq!(frequency.as_ghz_f64(), 1.);

        let frequency = Frequency::from_ghz_f32(1.);
        assert_eq!(frequency.as_ghz(), 1);
        assert_eq!(frequency.as_ghz_f32(), 1.);
        assert_eq!(frequency.as_ghz_f64(), 1.);

        let frequency = Frequency::from_ghz_f64(1.);
        assert_eq!(frequency.as_ghz(), 1);
        assert_eq!(frequency.as_ghz_f32(), 1.);
        assert_eq!(frequency.as_ghz_f64(), 1.);
    }

    #[test]
    fn add() {
        let freq = Frequency::from_hz(1) + Frequency::from_hz(1);
        assert_eq!(freq.as_hz(), 2);

        let freq = Frequency::from_hz(1_000) + Frequency::from_khz(1);
        assert_eq!(freq.as_khz(), 2);

        let freq = Frequency::from_hz(1_000_000) + Frequency::from_mhz(1);
        assert_eq!(freq.as_mhz(), 2);

        let freq = Frequency::from_hz(1_000_000_000) + Frequency::from_ghz(1);
        assert_eq!(freq.as_ghz(), 2);
    }

    #[test]
    fn subtract() {
        let freq = Frequency::from_hz(3) - Frequency::from_hz(1);
        assert_eq!(freq.as_hz(), 2);

        let freq = Frequency::from_hz(3_000) - Frequency::from_khz(1);
        assert_eq!(freq.as_khz(), 2);

        let freq = Frequency::from_hz(3_000_000) - Frequency::from_mhz(1);
        assert_eq!(freq.as_mhz(), 2);

        let freq = Frequency::from_hz(3_000_000_000) - Frequency::from_ghz(1);
        assert_eq!(freq.as_ghz(), 2);
    }

    #[test]
    #[should_panic]
    fn subtract_larger_frequency() {
        let _ = Frequency::from_hz(1) - Frequency::from_ghz(1);
    }

    #[test]
    fn multiply() {
        let freq = Frequency::from_hz(1) * 2;
        assert_eq!(freq.as_hz(), 2);

        let freq = Frequency::from_khz(1) * 2;
        assert_eq!(freq.as_khz(), 2);

        let freq = Frequency::from_mhz(1) * 2;
        assert_eq!(freq.as_mhz(), 2);

        let freq = Frequency::from_ghz(1) * 2;
        assert_eq!(freq.as_ghz(), 2);
    }

    #[test]
    fn divide() {
        let freq = Frequency::from_hz(4) / 2;
        assert_eq!(freq.as_hz(), 2);

        let freq = Frequency::from_khz(4) / 2;
        assert_eq!(freq.as_khz(), 2);

        let freq = Frequency::from_mhz(4) / 2;
        assert_eq!(freq.as_mhz(), 2);

        let freq = Frequency::from_ghz(4) / 2;
        assert_eq!(freq.as_ghz(), 2);
    }

    #[test]
    #[should_panic]
    fn divide_by_zero() {
        let _ = Frequency::from_hz(1) / 0;
    }
}
