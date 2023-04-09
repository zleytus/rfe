use std::time::Duration;

use chrono::{DateTime, Utc};
use nom::bytes::complete::{tag, take};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
    common::{parsers::*, Frequency, MessageParseError},
    signal_generator::parsers::*,
};

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

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Config {
    pub start: Frequency,
    pub cw: Frequency,
    pub total_steps: u32,
    pub step: Frequency,
    pub attenuation: Attenuation,
    pub power_level: PowerLevel,
    pub sweep_power_steps: u16,
    pub start_attenuation: Attenuation,
    pub start_power_level: PowerLevel,
    pub stop_attenuation: Attenuation,
    pub stop_power_level: PowerLevel,
    pub rf_power: RfPower,
    pub sweep_delay: Duration,
    pub timestamp: DateTime<Utc>,
}

impl Config {
    pub(crate) const PREFIX: &'static [u8] = b"#C3-*:";
}

impl<'a> TryFrom<&'a [u8]> for Config {
    type Error = MessageParseError<'a>;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Config::PREFIX)(bytes)?;

        // Parse the start frequency
        let (bytes, start_khz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the cw frequency
        let (bytes, cw_khz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the total steps
        let (bytes, total_steps) = parse_num(4u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the step frequency
        let (bytes, step_khz) = parse_frequency(7u8)(bytes)?;

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
        let _ = parse_opt_line_ending(bytes)?;

        Ok(Config {
            start: Frequency::from_khz(start_khz),
            cw: Frequency::from_khz(cw_khz),
            total_steps,
            step: Frequency::from_khz(step_khz),
            attenuation,
            power_level,
            sweep_power_steps,
            start_attenuation,
            start_power_level,
            stop_attenuation,
            stop_power_level,
            rf_power,
            sweep_delay: Duration::from_millis(u64::from(sweep_delay_ms)),
            timestamp: Utc::now(),
        })
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct ConfigExp {
    pub start: Frequency,
    pub cw: Frequency,
    pub total_steps: u32,
    pub step: Frequency,
    pub power_dbm: f32,
    pub step_power_dbm: f32,
    pub start_power_dbm: f32,
    pub stop_power_dbm: f32,
    pub rf_power_on: bool,
    pub sweep_delay: Duration,
    pub timestamp: DateTime<Utc>,
}

impl ConfigExp {
    pub const PREFIX: &'static [u8] = b"#C5-*:";
}

impl<'a> TryFrom<&'a [u8]> for ConfigExp {
    type Error = MessageParseError<'a>;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Self::PREFIX)(bytes)?;

        // Parse the start frequency
        let (bytes, start_khz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the CW frequency
        let (bytes, cw_khz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the total steps
        let (bytes, total_steps) = parse_num(4u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the step frequency
        let (bytes, step_khz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the power
        let (bytes, power_dbm) = parse_num(5u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the step power
        let (bytes, step_power_dbm) = parse_num(5u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the start power
        let (bytes, start_power_dbm) = parse_num(5u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the stop power
        let (bytes, stop_power_dbm) = parse_num(5u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the rf power
        let (bytes, rf_power) = take(1u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the sweep delay
        let (bytes, sweep_delay_ms) = parse_sweep_delay_ms(bytes)?;

        // Consume any \r or \r\n line endings and make sure there aren't any bytes left
        let _ = parse_opt_line_ending(bytes)?;

        Ok(ConfigExp {
            start: Frequency::from_khz(start_khz),
            cw: Frequency::from_khz(cw_khz),
            total_steps,
            step: Frequency::from_khz(step_khz),
            power_dbm,
            step_power_dbm,
            start_power_dbm,
            stop_power_dbm,
            rf_power_on: rf_power[0] == b'0',
            sweep_delay: Duration::from_millis(u64::from(sweep_delay_ms)),
            timestamp: Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config() {
        let bytes = b"#C3-*:0510000,0186525,0005,0001000,0,3,0000,0,0,1,3,0,00100\r\n";
        let config = Config::try_from(bytes.as_ref()).unwrap();
        assert_eq!(config.start.as_hz(), 510_000_000);
        assert_eq!(config.cw.as_hz(), 186_525_000);
        assert_eq!(config.total_steps, 5);
        assert_eq!(config.step.as_hz(), 1_000_000);
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
