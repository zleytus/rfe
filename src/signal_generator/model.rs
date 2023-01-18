use num_enum::TryFromPrimitive;

use crate::Frequency;

#[derive(Debug, Copy, Clone, TryFromPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum Model {
    RfGen = 60,
    RfGenExpansion = 61,
}

impl Model {
    pub fn min_freq(&self) -> Frequency {
        match self {
            Self::RfGen => 23_400_000,
            Self::RfGenExpansion => 100_000,
        }
        .into()
    }

    pub fn max_freq(&self) -> Frequency {
        match self {
            Self::RfGen => 6_000_000_000,
            Self::RfGenExpansion => 6_000_000_000,
        }
        .into()
    }
}
