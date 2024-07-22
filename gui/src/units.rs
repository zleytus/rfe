use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Units {
    Hz,
    Khz,
    Mhz,
    Ghz,
}

impl Display for Units {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Hz => write!(f, "Hz"),
            Self::Khz => write!(f, "kHz"),
            Self::Mhz => write!(f, "MHz"),
            Self::Ghz => write!(f, "GHz"),
        }
    }
}
