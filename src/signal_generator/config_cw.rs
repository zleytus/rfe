use crate::{
    rf_explorer::{parsers::*, Frequency},
    signal_generator::{parsers::*, Attenuation, PowerLevel, RfPower},
};
use nom::{bytes::complete::tag, IResult};

#[derive(Debug, Copy, Clone)]
pub struct ConfigCw {
    pub cw_freq: Frequency,
    pub total_steps: u32,
    pub step_freq: Frequency,
    pub attenuation: Attenuation,
    pub power_level: PowerLevel,
    pub rf_power: RfPower,
}

impl ConfigCw {
    pub const PREFIX: &'static [u8] = b"#C3-G:";

    pub(crate) fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Self::PREFIX)(bytes)?;

        // Parse the CW frequency
        let (bytes, cw_freq_khz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // The CW frequency is sent twice. Ignore the second occurrence.
        let (bytes, _): (_, u64) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the total steps
        let (bytes, total_steps) = parse_num(4u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the step frequency
        let (bytes, step_freq_khz) = parse_frequency(7u8)(bytes)?;

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
        let (bytes, _) = parse_opt_line_ending(bytes)?;

        Ok((
            bytes,
            ConfigCw {
                cw_freq: Frequency::from_khz(cw_freq_khz),
                total_steps,
                step_freq: Frequency::from_khz(step_freq_khz),
                attenuation,
                power_level,
                rf_power,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_cw() {
        let bytes = b"#C3-G:0186525,0186525,0005,0001000,0,3,0\r\n";
        let config_cw = ConfigCw::parse(bytes.as_ref()).unwrap().1;
        assert_eq!(config_cw.cw_freq.as_khz(), 186_525);
        assert_eq!(config_cw.total_steps, 5);
        assert_eq!(config_cw.step_freq.as_khz(), 1_000);
        assert_eq!(config_cw.attenuation, Attenuation::On);
        assert_eq!(config_cw.power_level, PowerLevel::Highest);
        assert_eq!(config_cw.rf_power, RfPower::On);
    }
}
