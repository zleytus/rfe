use std::fmt::Display;

use num_enum::TryFromPrimitive;

use crate::Frequency;

#[derive(Debug, Copy, Clone, TryFromPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum Model {
    Rfe6Gen = 60,
    Rfe6GenExpansion = 61,
}

impl Model {
    pub fn min_freq(&self) -> Frequency {
        match self {
            Self::Rfe6Gen => 23_400_000,
            Self::Rfe6GenExpansion => 100_000,
        }
        .into()
    }

    pub fn max_freq(&self) -> Frequency {
        match self {
            Self::Rfe6Gen => 6_000_000_000,
            Self::Rfe6GenExpansion => 6_000_000_000,
        }
        .into()
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Model::Rfe6Gen => write!(f, "6Gen"),
            Model::Rfe6GenExpansion => write!(f, "6Gen Expansion"),
        }
    }
}
