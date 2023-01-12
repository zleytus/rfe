use super::{Attenuation, PowerLevel};
use crate::common::Frequency;
use std::{borrow::Cow, time::Duration};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Command {
    RfPowerOn,
    RfPowerOff,
    StartAmpSweep {
        cw: Frequency,
        start_attenuation: Attenuation,
        start_power_level: PowerLevel,
        stop_attenuation: Attenuation,
        stop_power_level: PowerLevel,
        step_delay: Duration,
    },
    StartAmpSweepExp {
        cw: Frequency,
        start_power_dbm: f64,
        step_power_db: f64,
        stop_power_dbm: f64,
        step_delay: Duration,
    },
    StartCw {
        cw: Frequency,
        attenuation: Attenuation,
        power_level: PowerLevel,
    },
    StartCwExp {
        cw: Frequency,
        power_dbm: f64,
    },
    StartFreqSweep {
        start: Frequency,
        attenuation: Attenuation,
        power_level: PowerLevel,
        sweep_steps: u16,
        step: Frequency,
        step_delay: Duration,
    },
    StartFreqSweepExp {
        start: Frequency,
        power_dbm: f64,
        sweep_steps: u16,
        step: Frequency,
        step_delay: Duration,
    },
    StartTracking {
        start: Frequency,
        attenuation: Attenuation,
        power_level: PowerLevel,
        sweep_steps: u16,
        step: Frequency,
    },
    StartTrackingExp {
        start: Frequency,
        power_dbm: f64,
        sweep_steps: u16,
        step: Frequency,
    },
    TrackingStep(u16),
}

impl From<Command> for Cow<'static, [u8]> {
    fn from(command: Command) -> Cow<'static, [u8]> {
        match command {
            Command::RfPowerOn => Cow::Borrowed(&[b'#', 5, b'C', b'P', b'1'][..]),
            Command::RfPowerOff => Cow::Borrowed(&[b'#', 5, b'C', b'P', b'0']),
            Command::StartAmpSweep {
                cw,
                start_attenuation,
                start_power_level,
                stop_attenuation,
                stop_power_level,
                step_delay,
            } => {
                let mut command = vec![b'#', 30];
                command.extend(
                    format!(
                        "C3-A:{:07.0},{},{},{},{},{:05}",
                        cw.as_khz(),
                        u8::from(start_attenuation),
                        u8::from(start_power_level),
                        u8::from(stop_attenuation),
                        u8::from(stop_power_level),
                        step_delay.as_millis()
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::StartAmpSweepExp {
                cw,
                start_power_dbm,
                step_power_db,
                stop_power_dbm,
                step_delay,
            } => {
                let mut command = vec![b'#', 38];
                command.extend(
                    format!(
                        "C5-A:{:07.0},{:+05.1},{:+05.1},{:05.1},{:05}",
                        cw.as_khz(),
                        start_power_dbm,
                        step_power_db,
                        stop_power_dbm,
                        step_delay.as_millis()
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::StartCw {
                cw,
                attenuation,
                power_level,
            } => {
                let mut command = vec![b'#', 18];
                command.extend(
                    format!(
                        "C3-F:{:07.0},{},{}",
                        cw.as_khz(),
                        u8::from(attenuation),
                        u8::from(power_level)
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::StartCwExp { cw, power_dbm } => {
                let mut command = vec![b'#', 20];
                command.extend(format!("C5-F:{:07.0},{:+05.1}", cw.as_khz(), power_dbm).bytes());
                Cow::Owned(command)
            }
            Command::StartFreqSweep {
                start,
                attenuation,
                power_level,
                sweep_steps,
                step,
                step_delay,
            } => {
                let mut command = vec![b'#', 37];
                command.extend(
                    format!(
                        "C3-F:{:07.0},{},{},{:04},{:07.0},{:05}",
                        start.as_khz(),
                        u8::from(attenuation),
                        u8::from(power_level),
                        sweep_steps,
                        step.as_khz(),
                        step_delay.as_millis()
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::StartFreqSweepExp {
                start,
                power_dbm,
                sweep_steps,
                step,
                step_delay,
            } => {
                let mut command = vec![b'#', 39];
                command.extend(
                    format!(
                        "C5-F:{:07.0},{:+05.1},{:04},{:07.0},{:05}",
                        start.as_khz(),
                        power_dbm,
                        sweep_steps,
                        step.as_khz(),
                        step_delay.as_millis()
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::StartTracking {
                start,
                attenuation,
                power_level,
                sweep_steps,
                step,
            } => {
                let mut command = vec![b'#', 31];
                command.extend(
                    format!(
                        "C3-T:{:07.0},{},{},{:04},{:07.0}",
                        start.as_khz(),
                        u8::from(attenuation),
                        u8::from(power_level),
                        sweep_steps,
                        step.as_khz()
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::StartTrackingExp {
                start,
                power_dbm,
                sweep_steps,
                step,
            } => {
                let mut command = vec![b'#', 33];
                command.extend(
                    format!(
                        "C5-T:{:07.0},{:+05.1},{:04},{:07.0}",
                        start.as_khz(),
                        power_dbm,
                        sweep_steps,
                        step.as_khz()
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::TrackingStep(steps) => {
                let steps_bytes = steps.to_be_bytes();
                Cow::Owned(vec![b'#', 5, b'k', steps_bytes[0], steps_bytes[1]])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_correct_size {
        ($command:expr) => {
            let command_bytes = Cow::from($command);
            assert_eq!(
                command_bytes[1],
                command_bytes.len() as u8,
                "Command: {:?}",
                String::from_utf8_lossy(&command_bytes)
            );
        };
    }

    #[ignore]
    #[test]
    fn correct_command_size_fields() {
        assert_correct_size!(Command::RfPowerOn);
        assert_correct_size!(Command::RfPowerOff);
        assert_correct_size!(Command::StartAmpSweep {
            cw: Frequency::from_khz(100_000),
            start_attenuation: Attenuation::On,
            start_power_level: PowerLevel::Low,
            stop_attenuation: Attenuation::Off,
            stop_power_level: PowerLevel::Highest,
            step_delay: Duration::from_secs(1),
        });
        assert_correct_size!(Command::StartAmpSweepExp {
            cw: Frequency::from_khz(100_000),
            start_power_dbm: -40.,
            step_power_db: 2.,
            stop_power_dbm: 0.,
            step_delay: Duration::from_secs(1),
        });
        assert_correct_size!(Command::StartCw {
            cw: Frequency::from_mhz(1),
            attenuation: Attenuation::Off,
            power_level: PowerLevel::Low
        });
        assert_correct_size!(Command::StartCwExp {
            cw: Frequency::from_ghz(1),
            power_dbm: 10.
        });
        assert_correct_size!(Command::StartFreqSweep {
            start: Frequency::from_ghz(1),
            attenuation: Attenuation::Off,
            power_level: PowerLevel::High,
            sweep_steps: 10,
            step: Frequency::from_mhz(1),
            step_delay: Duration::from_secs(2)
        });
    }
}
