use super::{Attenuation, PowerLevel};
use crate::rf_explorer::Frequency;
use std::{borrow::Cow, time::Duration};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Command {
    RfPowerOn,
    RfPowerOff,
    StartAmpSweep {
        cw_freq: Frequency,
        start_attenuation: Attenuation,
        start_power_level: PowerLevel,
        stop_attenuation: Attenuation,
        stop_power_level: PowerLevel,
        step_delay: Duration,
    },
    StartAmpSweepExp {
        cw_freq: Frequency,
        start_power_dbm: f64,
        step_power_db: f64,
        stop_power_dbm: f64,
        step_delay: Duration,
    },
    StartCw {
        cw_freq: Frequency,
        attenuation: Attenuation,
        power_level: PowerLevel,
    },
    StartCwExp {
        cw_freq: Frequency,
        power_dbm: f64,
    },
    StartFreqSweep {
        start_freq: Frequency,
        attenuation: Attenuation,
        power_level: PowerLevel,
        sweep_steps: u16,
        step_freq: Frequency,
        step_delay: Duration,
    },
    StartFreqSweepExp {
        start_freq: Frequency,
        power_dbm: f64,
        sweep_steps: u16,
        step_freq: Frequency,
        step_delay: Duration,
    },
    StartTracking {
        start_freq: Frequency,
        attenuation: Attenuation,
        power_level: PowerLevel,
        sweep_steps: u16,
        step_freq: Frequency,
    },
    StartTrackingExp {
        start_freq: Frequency,
        power_dbm: f64,
        sweep_steps: u16,
        step_freq: Frequency,
    },
    TrackingStep(u16),
}

impl From<Command> for Cow<'static, [u8]> {
    fn from(command: Command) -> Cow<'static, [u8]> {
        match command {
            Command::RfPowerOn => Cow::Borrowed(&[b'#', 5, b'C', b'P', b'1'][..]),
            Command::RfPowerOff => Cow::Borrowed(&[b'#', 5, b'C', b'P', b'0']),
            Command::StartAmpSweep {
                cw_freq,
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
                        cw_freq.as_khz(),
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
                cw_freq,
                start_power_dbm,
                step_power_db,
                stop_power_dbm,
                step_delay,
            } => {
                let mut command = vec![b'#', 38];
                command.extend(
                    format!(
                        "C5-A:{:07.0},{:+05.1},{:+05.1},{:05.1},{:05}",
                        cw_freq.as_khz(),
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
                cw_freq,
                attenuation,
                power_level,
            } => {
                let mut command = vec![b'#', 18];
                command.extend(
                    format!(
                        "C3-F:{:07.0},{},{}",
                        cw_freq.as_khz(),
                        u8::from(attenuation),
                        u8::from(power_level)
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::StartCwExp { cw_freq, power_dbm } => {
                let mut command = vec![b'#', 20];
                command
                    .extend(format!("C5-F:{:07.0},{:+05.1}", cw_freq.as_khz(), power_dbm).bytes());
                Cow::Owned(command)
            }
            Command::StartFreqSweep {
                start_freq,
                attenuation,
                power_level,
                sweep_steps,
                step_freq,
                step_delay,
            } => {
                let mut command = vec![b'#', 37];
                command.extend(
                    format!(
                        "C3-F:{:07.0},{},{},{:04},{:07.0},{:05}",
                        start_freq.as_khz(),
                        u8::from(attenuation),
                        u8::from(power_level),
                        sweep_steps,
                        step_freq.as_khz(),
                        step_delay.as_millis()
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::StartFreqSweepExp {
                start_freq,
                power_dbm,
                sweep_steps,
                step_freq,
                step_delay,
            } => {
                let mut command = vec![b'#', 39];
                command.extend(
                    format!(
                        "C5-F:{:07.0},{:+05.1},{:04},{:07.0},{:05}",
                        start_freq.as_khz(),
                        power_dbm,
                        sweep_steps,
                        step_freq.as_khz(),
                        step_delay.as_millis()
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::StartTracking {
                start_freq,
                attenuation,
                power_level,
                sweep_steps,
                step_freq,
            } => {
                let mut command = vec![b'#', 31];
                command.extend(
                    format!(
                        "C3-T:{:07.0},{},{},{:04},{:07.0}",
                        start_freq.as_khz(),
                        u8::from(attenuation),
                        u8::from(power_level),
                        sweep_steps,
                        step_freq.as_khz()
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::StartTrackingExp {
                start_freq,
                power_dbm,
                sweep_steps,
                step_freq,
            } => {
                let mut command = vec![b'#', 33];
                command.extend(
                    format!(
                        "C5-T:{:07.0},{:+05.1},{:04},{:07.0}",
                        start_freq.as_khz(),
                        power_dbm,
                        sweep_steps,
                        step_freq.as_khz()
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
