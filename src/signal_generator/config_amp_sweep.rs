use crate::{
    common::{parsers::*, Frequency},
    signal_generator::{parsers::*, Attenuation, PowerLevel, RfPower},
};
use nom::{bytes::complete::tag, IResult};
use std::time::Duration;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct ConfigAmpSweep {
    pub cw: Frequency,
    pub sweep_power_steps: u16,
    pub start_attenuation: Attenuation,
    pub start_power_level: PowerLevel,
    pub stop_attenuation: Attenuation,
    pub stop_power_level: PowerLevel,
    pub rf_power: RfPower,
    pub sweep_delay: Duration,
}

impl ConfigAmpSweep {
    pub const PREFIX: &'static [u8] = b"#C3-A:";

    pub(crate) fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Self::PREFIX)(bytes)?;

        // Parse the cw frequency
        let (bytes, cw_khz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the sweep power steps
        let (bytes, sweep_power_steps) = parse_num(4u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the start attenuation
        let (bytes, start_attenuation) = parse_attenuation(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the start power level
        let (bytes, start_power_level) = parse_power_level(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the stop attenuation
        let (bytes, stop_attenuation) = parse_attenuation(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the stop power level
        let (bytes, stop_power_level) = parse_power_level(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the rf power
        let (bytes, rf_power) = parse_rf_power(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the sweep delay
        let (bytes, sweep_delay_ms) = parse_sweep_delay_ms(bytes)?;

        // Consume any \r or \r\n line endings and make sure there aren't any bytes left
        let (bytes, _) = parse_opt_line_ending(bytes)?;

        Ok((
            bytes,
            ConfigAmpSweep {
                cw: Frequency::from_khz(cw_khz),
                sweep_power_steps,
                start_attenuation,
                start_power_level,
                stop_attenuation,
                stop_power_level,
                rf_power,
                sweep_delay: Duration::from_millis(u64::from(sweep_delay_ms)),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config() {
        let bytes = b"#C3-A:0186525,0000,0,0,1,3,0,00100\r\n";
        let config_amp_sweep = ConfigAmpSweep::parse(bytes.as_ref()).unwrap().1;
        assert_eq!(config_amp_sweep.cw.as_khz(), 186_525);
        assert_eq!(config_amp_sweep.sweep_power_steps, 0);
        assert_eq!(config_amp_sweep.start_attenuation, Attenuation::On);
        assert_eq!(config_amp_sweep.start_power_level, PowerLevel::Lowest);
        assert_eq!(config_amp_sweep.stop_attenuation, Attenuation::Off);
        assert_eq!(config_amp_sweep.stop_power_level, PowerLevel::Highest);
        assert_eq!(config_amp_sweep.rf_power, RfPower::On);
        assert_eq!(config_amp_sweep.sweep_delay.as_millis(), 100);
    }
}
