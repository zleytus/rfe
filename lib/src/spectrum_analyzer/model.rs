use std::fmt::Display;

use num_enum::TryFromPrimitive;

use crate::Frequency;

#[derive(Debug, Copy, Clone, TryFromPrimitive, Eq, PartialEq, Default)]
#[repr(u8)]
pub enum Model {
    Rfe433M = 0,
    Rfe868M = 1,
    Rfe915M = 2,
    RfeWSub1G = 3,
    Rfe24G = 4,
    RfeWSub3G = 5,
    Rfe6G = 6,
    RfeWSub1GPlus = 10,
    RfeProAudio = 11,
    Rfe24GPlus = 12,
    Rfe4GPlus = 13,
    Rfe6GPlus = 14,
    RfeMW5G3G = 16,
    RfeMW5G4G = 17,
    RfeMW5G5G = 18,
    #[default]
    #[num_enum(alternatives = [20..=254])]
    Unknown = 19,
}

impl Model {
    pub const fn is_plus_model(&self) -> bool {
        matches!(
            self,
            Model::RfeWSub1GPlus
                | Model::RfeProAudio
                | Model::Rfe24GPlus
                | Model::Rfe4GPlus
                | Model::Rfe6GPlus
                | Model::RfeMW5G3G
                | Model::RfeMW5G4G
                | Model::RfeMW5G5G
        )
    }

    pub const fn has_wifi_analyzer(&self) -> bool {
        matches!(
            self,
            Model::Rfe24G
                // The IoT module MWSub3G reports itself as being a WSub3G but does not support
                // the Wi-Fi analyzer mode unlike regular WSub3G models
                | Model::RfeWSub3G
                | Model::Rfe6G
                | Model::Rfe24GPlus
                | Model::Rfe4GPlus
                | Model::Rfe6GPlus
        )
    }

    pub fn min_freq(&self) -> Frequency {
        match self {
            Model::Rfe433M => 430_000_000,
            Model::Rfe868M => 860_000_000,
            Model::Rfe915M => 910_000_000,
            Model::RfeWSub1G => 240_000_000,
            Model::RfeWSub1GPlus => 50_000,
            Model::Rfe24G | Model::Rfe24GPlus => 2_350_000_000,
            Model::RfeWSub3G
            | Model::RfeProAudio
            | Model::RfeMW5G3G
            | Model::RfeMW5G4G
            | Model::RfeMW5G5G => 15_000_000,
            Model::Rfe6G => 4_850_000_000,
            Model::Rfe4GPlus | Model::Rfe6GPlus => 240_000_000,
            Model::Unknown => u64::MIN,
        }
        .into()
    }

    pub fn max_freq(&self) -> Frequency {
        match self {
            Model::Rfe433M => 440_000_000,
            Model::Rfe868M => 870_000_000,
            Model::Rfe915M => 920_000_000,
            Model::RfeWSub1G | Model::RfeWSub1GPlus => 960_000_000,
            Model::Rfe24G | Model::Rfe24GPlus => 2_550_000_000,
            Model::RfeWSub3G | Model::RfeProAudio => 2_700_000_000,
            Model::Rfe4GPlus => 4_000_000_000,
            Model::Rfe6G | Model::Rfe6GPlus => 6_100_000_000,
            Model::RfeMW5G3G => 3_000_000_000,
            Model::RfeMW5G4G => 4_000_000_000,
            Model::RfeMW5G5G => 5_000_000_000,
            Model::Unknown => u64::MAX,
        }
        .into()
    }

    pub fn min_span(&self) -> Frequency {
        match self {
            Model::Rfe433M
            | Model::Rfe868M
            | Model::Rfe915M
            | Model::RfeWSub1G
            | Model::RfeWSub3G
            | Model::RfeProAudio
            // The minimum spans of MW5G models is not documented so this could be incorrect
            | Model::RfeMW5G3G
            | Model::RfeMW5G4G
            | Model::RfeMW5G5G => 112_000,
            Model::RfeWSub1GPlus => 100_000,
            Model::Rfe24G
            | Model::Rfe24GPlus
            | Model::Rfe4GPlus
            | Model::Rfe6G
            | Model::Rfe6GPlus => 2_000_000,
            Model::Unknown => u64::MIN,
        }
        .into()
    }

    pub fn max_span(&self) -> Frequency {
        match self {
            Model::Rfe433M | Model::Rfe868M | Model::Rfe915M => 10_000_000,
            Model::Rfe24G | Model::Rfe24GPlus => 85_000_000,
            // The maximum spans of MW5G models is not documented so this could be incorrect
            Model::RfeWSub1G | Model::RfeMW5G3G | Model::RfeMW5G4G | Model::RfeMW5G5G => {
                300_000_000
            }
            Model::RfeWSub3G | Model::RfeProAudio | Model::Rfe6G => 600_000_000,
            Model::RfeWSub1GPlus => 959_950_000,
            Model::Rfe4GPlus | Model::Rfe6GPlus => 960_000_000,
            Model::Unknown => u64::MAX,
        }
        .into()
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Rfe433M => "433M",
                Self::Rfe868M => "868M",
                Self::Rfe915M => "915M",
                Self::RfeWSub1G => "WSUB1G",
                Self::Rfe24G => "2.4G",
                Self::RfeWSub3G => "WSUB3G",
                Self::Rfe6G => "6G",
                Self::RfeWSub1GPlus => "WSUB1G+",
                Self::RfeProAudio => "Pro Audio",
                Self::Rfe24GPlus => "2.4G+",
                Self::Rfe4GPlus => "4G+",
                Self::Rfe6GPlus => "6G+",
                Self::RfeMW5G3G => "MW5G 3GHz",
                Self::RfeMW5G4G => "MW5G 4GHz",
                Self::RfeMW5G5G => "MW5G 5GHz",
                Self::Unknown => "Unknown",
            }
        )
    }
}
