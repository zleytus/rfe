use std::time::Duration;

use chrono::{DateTime, Utc};
use nom::{
    bytes::complete::{tag, take},
    Parser,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
    common::{Frequency, MessageParseError},
    rf_explorer::parsers::*,
    signal_generator::parsers::*,
};

/// RF output attenuation state.
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum Attenuation {
    /// Attenuation is enabled.
    #[default]
    On = 0,
    /// Attenuation is disabled.
    Off,
}

/// Discrete RF output power level.
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum PowerLevel {
    /// Lowest output power.
    #[default]
    Lowest = 0,
    /// Low output power.
    Low,
    /// High output power.
    High,
    /// Highest output power.
    Highest,
}

/// RF output power state.
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum RfPower {
    /// RF output is enabled.
    On = 0,
    /// RF output is disabled.
    #[default]
    Off,
}

/// Main-module signal generator configuration.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Config {
    /// Start frequency for frequency sweep and tracking modes.
    pub start: Frequency,
    /// CW frequency.
    pub cw: Frequency,
    /// Total number of sweep or tracking steps.
    pub total_steps: u32,
    /// Frequency increment per step.
    pub step: Frequency,
    /// CW and frequency sweep attenuation setting.
    pub attenuation: Attenuation,
    /// CW and frequency sweep power level.
    pub power_level: PowerLevel,
    /// Number of amplitude sweep power steps.
    pub sweep_power_steps: u16,
    /// Amplitude sweep start attenuation setting.
    pub start_attenuation: Attenuation,
    /// Amplitude sweep start power level.
    pub start_power_level: PowerLevel,
    /// Amplitude sweep stop attenuation setting.
    pub stop_attenuation: Attenuation,
    /// Amplitude sweep stop power level.
    pub stop_power_level: PowerLevel,
    /// RF output power state.
    pub rf_power: RfPower,
    /// Delay between sweep steps.
    pub sweep_delay: Duration,
    /// Time when this configuration was received.
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
        let (bytes, start_khz) = freq_parser(7u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the cw frequency
        let (bytes, cw_khz) = freq_parser(7u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the total steps
        let (bytes, total_steps) = num_parser(4u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the step frequency
        let (bytes, step_khz) = freq_parser(7u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the attenuation
        let (bytes, attenuation) = parse_attenuation(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the power level
        let (bytes, power_level) = parse_power_level(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the sweep power steps
        let (bytes, sweep_power_steps) = num_parser(4u8).parse(bytes)?;

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

/// Expansion-module signal generator configuration.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct ConfigExp {
    /// Start frequency for frequency sweep and tracking modes.
    pub start: Frequency,
    /// CW frequency.
    pub cw: Frequency,
    /// Total number of sweep or tracking steps.
    pub total_steps: u32,
    /// Frequency increment per step.
    pub step: Frequency,
    /// Current output power in dBm.
    pub power_dbm: f32,
    /// Power increment per amplitude sweep step in dB.
    pub step_power_dbm: f32,
    /// Amplitude sweep start power in dBm.
    pub start_power_dbm: f32,
    /// Amplitude sweep stop power in dBm.
    pub stop_power_dbm: f32,
    /// Whether RF output power is enabled.
    pub rf_power_on: bool,
    /// Delay between sweep steps.
    pub sweep_delay: Duration,
    /// Time when this configuration was received.
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
        let (bytes, start_khz) = freq_parser(7u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the CW frequency
        let (bytes, cw_khz) = freq_parser(7u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the total steps
        let (bytes, total_steps) = num_parser(4u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the step frequency
        let (bytes, step_khz) = freq_parser(7u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the power
        let (bytes, power_dbm) = num_parser(5u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the step power
        let (bytes, step_power_dbm) = num_parser(5u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the start power
        let (bytes, start_power_dbm) = num_parser(5u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the stop power
        let (bytes, stop_power_dbm) = num_parser(5u8).parse(bytes)?;

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
