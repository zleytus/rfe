use std::fmt::Display;

use num_enum::TryFromPrimitive;

use crate::Frequency;

/// Signal generator model reported by the RF Explorer.
#[derive(Debug, Copy, Clone, TryFromPrimitive, Eq, PartialEq, Default)]
#[repr(u8)]
pub enum Model {
    /// Main 6 GHz signal generator module.
    #[default]
    Rfe6Gen = 60,
    /// Expansion 6 GHz signal generator module.
    Rfe6GenExpansion = 61,
}

impl Model {
    /// Returns the model's minimum supported output frequency.
    pub fn min_freq(&self) -> Frequency {
        match self {
            Self::Rfe6Gen => 23_400_000,
            Self::Rfe6GenExpansion => 100_000,
        }
        .into()
    }

    /// Returns the model's maximum supported output frequency.
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
