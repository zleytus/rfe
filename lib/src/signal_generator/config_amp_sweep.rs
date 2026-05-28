use std::time::Duration;

use chrono::{DateTime, Utc};
use nom::{Parser, bytes::complete::tag};

use crate::{
    common::{Frequency, MessageParseError},
    rf_explorer::parsers::*,
    signal_generator::{Attenuation, PowerLevel, RfPower, parsers::*},
};

/// Main-module amplitude sweep configuration.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct ConfigAmpSweep {
    /// CW frequency used during the amplitude sweep.
    pub cw: Frequency,
    /// Number of power steps in the sweep.
    pub sweep_power_steps: u16,
    /// Starting attenuation setting.
    pub start_attenuation: Attenuation,
    /// Starting output power level.
    pub start_power_level: PowerLevel,
    /// Stopping attenuation setting.
    pub stop_attenuation: Attenuation,
    /// Stopping output power level.
    pub stop_power_level: PowerLevel,
    /// RF output power state.
    pub rf_power: RfPower,
    /// Delay between amplitude sweep steps.
    pub sweep_delay: Duration,
    /// Time when this configuration was received.
    pub timestamp: DateTime<Utc>,
}

impl ConfigAmpSweep {
    pub(crate) const PREFIX: &'static [u8] = b"#C3-A:";
}

impl<'a> TryFrom<&'a [u8]> for ConfigAmpSweep {
    type Error = MessageParseError<'a>;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Self::PREFIX)(bytes)?;

        // Parse the cw frequency
        let (bytes, cw_khz) = freq_parser(7u8).parse(bytes)?;

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

        Ok(ConfigAmpSweep {
            cw: Frequency::from_khz(cw_khz),
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

/// Expansion-module amplitude sweep configuration.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct ConfigAmpSweepExp {
    /// CW frequency used during the amplitude sweep.
    pub cw: Frequency,
    /// Starting output power in dBm.
    pub start_power_dbm: f32,
    /// Power increment per step in dB.
    pub step_power_dbm: f32,
    /// Stopping output power in dBm.
    pub stop_power_dbm: f32,
    /// Delay between amplitude sweep steps.
    pub sweep_delay: Duration,
    /// Time when this configuration was received.
    pub timestamp: DateTime<Utc>,
}

impl ConfigAmpSweepExp {
    pub const PREFIX: &'static [u8] = b"#C5-A:";
}

impl<'a> TryFrom<&'a [u8]> for ConfigAmpSweepExp {
    type Error = MessageParseError<'a>;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Self::PREFIX)(bytes)?;

        // Parse the CW frequency
        let (bytes, cw_khz) = num_parser(7u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the start power
        let (bytes, start_power_dbm) = num_parser(5u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the step power
        let (bytes, step_power_dbm) = num_parser(5u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the stop power
        let (bytes, stop_power_dbm) = num_parser(5u8).parse(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the sweep delay
        let (bytes, sweep_delay_ms) = parse_sweep_delay_ms(bytes)?;

        // Consume any \r or \r\n line endings and make sure there aren't any bytes left
        let _ = parse_opt_line_ending(bytes)?;

        Ok(ConfigAmpSweepExp {
            cw: Frequency::from_khz(cw_khz),
            start_power_dbm,
            step_power_dbm,
            stop_power_dbm,
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
        let bytes = b"#C3-A:0186525,0000,0,0,1,3,0,00100\r\n";
        let config_amp_sweep = ConfigAmpSweep::try_from(bytes.as_ref()).unwrap();
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
