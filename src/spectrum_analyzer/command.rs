use std::borrow::Cow;

use super::{CalcMode, DspMode, InputStage, WifiBand};
use crate::rf_explorer::Frequency;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Command {
    SetConfig {
        start_freq: Frequency,
        stop_freq: Frequency,
        min_amp_dbm: i16,
        max_amp_dbm: i16,
    },
    SwitchModuleMain,
    SwitchModuleExp,
    StartTracking {
        start_freq: Frequency,
        step_freq: Frequency,
    },
    StartWifiAnalyzer(WifiBand),
    StopWifiAnalyzer,
    SetCalcMode(CalcMode),
    TrackingStep(u16),
    SetDsp(DspMode),
    SetOffsetDB(i8),
    SetInputStage(InputStage),
    SetSweepPointsExt(u16),
    SetSweepPointsLarge(u16),
}

impl From<Command> for Cow<'static, [u8]> {
    fn from(command: Command) -> Cow<'static, [u8]> {
        match command {
            Command::SetConfig {
                start_freq,
                stop_freq,
                min_amp_dbm,
                max_amp_dbm,
            } => {
                let mut command = vec![b'#', 32];
                command.extend(
                    format!(
                        "C2-F:{:07.0},{:07.0},{:04},{:04}",
                        start_freq.as_khz(),
                        stop_freq.as_khz(),
                        max_amp_dbm,
                        min_amp_dbm
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::SwitchModuleMain => Cow::Borrowed(&[b'#', 5, b'C', b'M', 0]),
            Command::SwitchModuleExp => Cow::Borrowed(&[b'#', 5, b'C', b'M', 1]),
            Command::StartTracking {
                start_freq,
                step_freq,
            } => {
                let mut command = vec![b'#', 22];
                command.extend(
                    format!(
                        "C3-K:{:07.0},{:07.0}",
                        start_freq.as_khz(),
                        step_freq.as_khz()
                    )
                    .bytes(),
                );
                Cow::Owned(command)
            }
            Command::StartWifiAnalyzer(wifi_band) => {
                Cow::Owned(vec![b'#', 5, b'C', b'W', u8::from(wifi_band)])
            }
            Command::StopWifiAnalyzer => Cow::Owned(vec![b'#', 5, b'C', b'W', 0]),
            Command::SetCalcMode(calc_mode) => {
                Cow::Owned(vec![b'#', 5, b'C', b'+', u8::from(calc_mode)])
            }
            Command::TrackingStep(steps) => {
                let steps_bytes = steps.to_be_bytes();
                Cow::Owned(vec![b'#', 5, b'k', steps_bytes[0], steps_bytes[1]])
            }
            Command::SetDsp(dsp_mode) => Cow::Owned(vec![b'#', 5, b'C', b'p', u8::from(dsp_mode)]),
            Command::SetOffsetDB(offset_db) => {
                Cow::Owned(vec![b'#', 5, b'C', b'O', offset_db as u8])
            }
            Command::SetInputStage(input_stage) => {
                Cow::Owned(vec![b'#', 4, b'a', u8::from(input_stage)])
            }
            Command::SetSweepPointsExt(sweep_points) => {
                Cow::Owned(vec![b'#', 5, b'C', b'J', ((sweep_points / 16) - 1) as u8])
            }
            Command::SetSweepPointsLarge(sweep_points) => {
                let sweep_point_bytes = sweep_points.to_be_bytes();
                Cow::Owned(vec![
                    b'#',
                    6,
                    b'C',
                    b'j',
                    sweep_point_bytes[0],
                    sweep_point_bytes[1],
                ])
            }
        }
    }
}
