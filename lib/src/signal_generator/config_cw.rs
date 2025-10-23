use chrono::{DateTime, Utc};
use nom::{bytes::complete::tag, Parser};

use crate::{
    common::{Frequency, MessageParseError},
    rf_explorer::parsers::*,
    signal_generator::{parsers::*, Attenuation, PowerLevel, RfPower},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct ConfigCw {
    pub cw: Frequency,
    pub total_steps: u32,
    pub step_freq: Frequency,
    pub attenuation: Attenuation,
    pub power_level: PowerLevel,
    pub rf_power: RfPower,
    pub timestamp: DateTime<Utc>,
}

impl ConfigCw {
    pub(crate) const PREFIX: &'static [u8] = b"#C3-G:";
}

impl<'a> TryFrom<&'a [u8]> for ConfigCw {
    type Error = MessageParseError<'a>;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Self::PREFIX)(bytes)?;

        // Parse the CW frequency
        let (bytes, cw_khz) = parse_frequency(7u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // The CW frequency is sent twice. Ignore the second occurrence.
        let (bytes, _): (_, u64) = parse_frequency(7u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the total steps
        let (bytes, total_steps) = parse_num(4u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the step frequency
        let (bytes, step_khz) = parse_frequency(7u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the attenuation
        let (bytes, attenuation) = parse_attenuation(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the power level
        let (bytes, power_level) = parse_power_level(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the rf power
        let (bytes, rf_power) = parse_rf_power(bytes)?;

        // Consume any \r or \r\n line endings and make sure there aren't any bytes left
        let _ = parse_opt_line_ending(bytes)?;

        Ok(ConfigCw {
            cw: Frequency::from_khz(cw_khz),
            total_steps,
            step_freq: Frequency::from_khz(step_khz),
            attenuation,
            power_level,
            rf_power,
            timestamp: Utc::now(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct ConfigCwExp {
    pub cw: Frequency,
    pub power_dbm: f32,
    pub rf_power: RfPower,
    pub timestamp: DateTime<Utc>,
}

impl ConfigCwExp {
    pub const PREFIX: &'static [u8] = b"#C5-G:";
}

impl<'a> TryFrom<&'a [u8]> for ConfigCwExp {
    type Error = MessageParseError<'a>;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Self::PREFIX)(bytes)?;

        // Parse the CW freq
        let (bytes, cw_khz) = parse_frequency(7u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the power
        let (bytes, power_dbm) = parse_num(5u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the rf power
        let (bytes, rf_power) = parse_rf_power(bytes)?;

        // Consume any \r or \r\n line endings and make sure there aren't any bytes left
        let _ = parse_opt_line_ending(bytes)?;

        Ok(ConfigCwExp {
            cw: Frequency::from_khz(cw_khz),
            power_dbm,
            rf_power,
            timestamp: Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_cw() {
        let bytes = b"#C3-G:0186525,0186525,0005,0001000,0,3,0\r\n";
        let config_cw = ConfigCw::try_from(bytes.as_ref()).unwrap();
        assert_eq!(config_cw.cw.as_khz(), 186_525);
        assert_eq!(config_cw.total_steps, 5);
        assert_eq!(config_cw.step_freq.as_khz(), 1_000);
        assert_eq!(config_cw.attenuation, Attenuation::On);
        assert_eq!(config_cw.power_level, PowerLevel::Highest);
        assert_eq!(config_cw.rf_power, RfPower::On);
    }
}
