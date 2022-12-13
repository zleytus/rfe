use crate::{
    rf_explorer::{parsers::*, Frequency},
    signal_generator::parsers::*,
};
use nom::{bytes::complete::tag, IResult};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::time::Duration;

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum Attenuation {
    #[default]
    On = 0,
    Off,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum PowerLevel {
    #[default]
    Lowest = 0,
    Low,
    High,
    Highest,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum RfPower {
    On = 0,
    #[default]
    Off,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Config {
    pub start_freq: Frequency,
    pub cw_freq: Frequency,
    pub total_steps: u32,
    pub step_freq: Frequency,
    pub attenuation: Attenuation,
    pub power_level: PowerLevel,
    pub sweep_power_steps: u16,
    pub start_attenuation: Attenuation,
    pub start_power_level: PowerLevel,
    pub stop_attenuation: Attenuation,
    pub stop_power_level: PowerLevel,
    pub rf_power: RfPower,
    pub sweep_delay: Duration,
}

impl Config {
    pub const PREFIX: &'static [u8] = b"#C3-*:";

    pub(crate) fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Config::PREFIX)(bytes)?;

        // Parse the start frequency
        let (bytes, start_freq_khz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the cw frequency
        let (bytes, cw_freq_khz) = parse_frequency(7u8)(bytes)?;

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
            Config {
                start_freq: Frequency::from_khz(start_freq_khz),
                cw_freq: Frequency::from_khz(cw_freq_khz),
                total_steps,
                step_freq: Frequency::from_khz(step_freq_khz),
                attenuation,
                power_level,
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
        let bytes = b"#C3-*:0510000,0186525,0005,0001000,0,3,0000,0,0,1,3,0,00100\r\n";
        let config = Config::parse(bytes.as_ref()).unwrap().1;
        assert_eq!(config.start_freq.as_hz(), 510_000_000);
        assert_eq!(config.cw_freq.as_hz(), 186_525_000);
        assert_eq!(config.total_steps, 5);
        assert_eq!(config.step_freq.as_hz(), 1_000_000);
        assert_eq!(config.attenuation, Attenuation::On);
        assert_eq!(config.power_level, PowerLevel::Highest);
        assert_eq!(config.sweep_power_steps, 0);
        assert_eq!(config.start_attenuation, Attenuation::On);
        assert_eq!(config.start_power_level, PowerLevel::Lowest);
        assert_eq!(config.stop_attenuation, Attenuation::Off);
        assert_eq!(config.stop_power_level, PowerLevel::Highest);
        assert_eq!(config.rf_power, RfPower::On);
        assert_eq!(config.sweep_delay.as_millis(), 100);
    }
}
