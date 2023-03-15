use std::time::Duration;

use chrono::{DateTime, Utc};
use nom::bytes::complete::tag;

use crate::{
    common::{parsers::*, Frequency, MessageParseError},
    signal_generator::{parsers::*, Attenuation, PowerLevel, RfPower},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct ConfigFreqSweep {
    pub start: Frequency,
    pub total_steps: u32,
    pub step: Frequency,
    pub attenuation: Attenuation,
    pub power_level: PowerLevel,
    pub rf_power: RfPower,
    pub sweep_delay: Duration,
    pub timestamp: DateTime<Utc>,
}
impl ConfigFreqSweep {
    pub const PREFIX: &'static [u8] = b"#C3-F:";
}

impl<'a> TryFrom<&'a [u8]> for ConfigFreqSweep {
    type Error = MessageParseError<'a>;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Self::PREFIX)(bytes)?;

        // Parse the start frequency
        let (bytes, start_khz) = parse_frequency(7u8)(bytes)?;

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

        // Parse the rf power
        let (bytes, rf_power) = parse_rf_power(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the sweep delay
        let (bytes, sweep_delay_ms) = parse_sweep_delay_ms(bytes)?;

        // Consume any \r or \r\n line endings and make sure there aren't any bytes left
        let _ = parse_opt_line_ending(bytes)?;

        Ok(ConfigFreqSweep {
            start: Frequency::from_khz(start_khz),
            total_steps,
            step: Frequency::from_khz(step_khz),
            attenuation,
            power_level,
            rf_power,
            sweep_delay: Duration::from_millis(u64::from(sweep_delay_ms)),
            timestamp: Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_freq_sweep() {
        let bytes = b"#C3-F:0186525,0005,0001000,0,3,0,00100";
        let config_freq_sweep = ConfigFreqSweep::try_from(bytes.as_ref()).unwrap();
        assert_eq!(config_freq_sweep.start.as_khz(), 186_525);
        assert_eq!(config_freq_sweep.total_steps, 5);
        assert_eq!(config_freq_sweep.step.as_khz(), 1_000);
        assert_eq!(config_freq_sweep.attenuation, Attenuation::On);
        assert_eq!(config_freq_sweep.power_level, PowerLevel::Highest);
        assert_eq!(config_freq_sweep.rf_power, RfPower::On);
        assert_eq!(config_freq_sweep.sweep_delay.as_millis(), 100);
    }
}
