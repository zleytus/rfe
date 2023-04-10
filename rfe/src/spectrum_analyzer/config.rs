use std::fmt::Display;

use chrono::{DateTime, Utc};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map_res, opt},
};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
    common::{parsers::*, Frequency, MessageParseError},
    spectrum_analyzer::parsers::*,
};

#[derive(Debug, Copy, Clone, TryFromPrimitive, Eq, PartialEq, Default)]
#[repr(u8)]
pub enum Mode {
    #[default]
    SpectrumAnalyzer = 0,
    RfGenerator = 1,
    WifiAnalyzer = 2,
    AnalyzerTracking = 5,
    RfSniffer = 6,
    CwTransmitter = 60,
    SweepFrequency = 61,
    SweepAmplitude = 62,
    GeneratorTracking = 63,
    Unknown = 255,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            Mode::SpectrumAnalyzer => "Spectrum Analyzer",
            Mode::RfGenerator => "RF Generator",
            Mode::WifiAnalyzer => "Wi-Fi Analyzer",
            Mode::AnalyzerTracking => "Analyzer Tracking",
            Mode::RfSniffer => "RF Sniffer",
            Mode::CwTransmitter => "CW Transmitter",
            Mode::SweepFrequency => "Sweep Frequency",
            Mode::SweepAmplitude => "Sweep Amplitude",
            Mode::GeneratorTracking => "Generator Tracking",
            Mode::Unknown => "Unknown",
        };
        write!(f, "{mode}")
    }
}

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive, Eq, PartialEq, Default)]
#[repr(u8)]
pub enum CalcMode {
    #[default]
    Normal = 0,
    Max,
    Avg,
    Overwrite,
    MaxHold,
    MaxHistorical,
    Unknown = 255,
}

impl Display for CalcMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let calc_mode = match self {
            CalcMode::Normal => "Normal",
            CalcMode::Max => "Max",
            CalcMode::Avg => "Average",
            CalcMode::Overwrite => "Overwrite",
            CalcMode::MaxHold => "Max Hold",
            CalcMode::MaxHistorical => "Max Historical",
            CalcMode::Unknown => "Unknown",
        };
        write!(f, "{calc_mode}")
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Config {
    pub start: Frequency,
    pub step: Frequency,
    pub stop: Frequency,
    pub center: Frequency,
    pub span: Frequency,
    pub max_amp_dbm: i16,
    pub min_amp_dbm: i16,
    pub sweep_points: u16,
    pub is_expansion_radio_module_active: bool,
    pub mode: Mode,
    pub min_freq: Frequency,
    pub max_freq: Frequency,
    pub max_span: Frequency,
    pub rbw: Option<Frequency>,
    pub amp_offset_db: Option<i8>,
    pub calc_mode: Option<CalcMode>,
    pub timestamp: DateTime<Utc>,
}

impl Config {
    pub(crate) const PREFIX: &'static [u8] = b"#C2-F:";

    #[tracing::instrument(skip(self), ret, fields(self.start = ?self.start, self.stop = ?self.stop, self.min_amp_dbm = ?self.min_amp_dbm, self.max_amp_dbm = ?self.max_amp_dbm))]
    pub(crate) fn contains_start_stop_amp_range(
        &self,
        start: Frequency,
        stop: Frequency,
        min_amp_dbm: i16,
        max_amp_dbm: i16,
    ) -> bool {
        self.start.abs_diff(start) <= self.step
            && self.stop.abs_diff(stop) <= self.step * 2
            && self.min_amp_dbm == min_amp_dbm
            && self.max_amp_dbm == max_amp_dbm
    }
}

impl<'a> TryFrom<&'a [u8]> for Config {
    type Error = MessageParseError<'a>;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Config::PREFIX)(bytes)?;

        // Parse the start frequency
        let (bytes, start_khz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the step frequency
        let (bytes, step_hz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the max amplitude
        let (bytes, max_amp_dbm) = parse_amplitude(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the min amplitude
        let (bytes, min_amp_dbm) = parse_amplitude(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the number of points in a sweep
        // 0-9999 uses 4 bytes and 10000+ uses 5 bytes
        // Try to parse using 5 bytes first and if that doesn't work fall back to 4 bytes
        let (bytes, sweep_points) = alt((parse_num(5u8), parse_num(4u8)))(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse whether or not the expansion module is active
        let (bytes, is_expansion_radio_module_active) = map_res(parse_num(1), |num| match num {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(()),
        })(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the mode
        let (bytes, mode) = parse_mode(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the minimum frequency
        let (bytes, min_freq_khz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the maximum frequency
        let (bytes, max_freq_khz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = parse_comma(bytes)?;

        // Parse the maximum span
        let (bytes, max_span_khz) = parse_frequency(7u8)(bytes)?;

        let (bytes, _) = opt(parse_comma)(bytes)?;

        // Parse the RBW
        // This field is optional because it's not sent by older RF Explorers
        let (bytes, rbw_khz) = opt(parse_frequency(5u8))(bytes)?;

        let (bytes, _) = opt(parse_comma)(bytes)?;

        // Parse the amplitude offset
        // This field is optional because it's not sent by older RF Explorers
        let (bytes, amp_offset_db) = opt(parse_amplitude)(bytes)?;

        let (bytes, _) = opt(parse_comma)(bytes)?;

        // Parse the calculator mode
        // This field is optional because it's not sent by older RF Explorers
        let (bytes, calc_mode) = opt(parse_calc_mode)(bytes)?;

        // Consume \n or \r\n line endings and make sure there aren't any bytes left afterwards
        let _ = parse_opt_line_ending(bytes)?;

        let start = Frequency::from_khz(start_khz);
        let step = Frequency::from_hz(step_hz);
        let stop = start + (step * u64::from(sweep_points - 1));

        Ok(Config {
            start,
            stop,
            step,
            center: (start + stop) / 2,
            span: stop - start,
            max_amp_dbm,
            min_amp_dbm,
            sweep_points,
            is_expansion_radio_module_active,
            mode,
            min_freq: Frequency::from_khz(min_freq_khz),
            max_freq: Frequency::from_khz(max_freq_khz),
            max_span: Frequency::from_khz(max_span_khz),
            rbw: rbw_khz.map(Frequency::from_khz),
            amp_offset_db,
            calc_mode,
            timestamp: Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_6g_combo_config() {
        let bytes =
            b"#C2-F:5249000,0196428,-030,-118,0112,0,000,4850000,6100000,0600000,00200,0000,000";
        let config = Config::try_from(bytes.as_ref()).unwrap();
        assert_eq!(config.start.as_hz(), 5_249_000_000);
        assert_eq!(config.step.as_hz(), 196_428);
        assert_eq!(config.stop.as_hz(), 5_270_803_508);
        assert_eq!(config.center.as_hz(), 5_259_901_754);
        assert_eq!(config.span.as_hz(), 21_803_508);
        assert_eq!(config.max_amp_dbm, -30);
        assert_eq!(config.min_amp_dbm, -118);
        assert_eq!(config.sweep_points, 112);
        assert!(!config.is_expansion_radio_module_active);
        assert_eq!(config.mode, Mode::SpectrumAnalyzer);
        assert_eq!(config.min_freq.as_hz(), 4_850_000_000);
        assert_eq!(config.max_freq.as_hz(), 6_100_000_000);
        assert_eq!(config.max_span.as_hz(), 600_000_000);
        assert_eq!(config.rbw, Some(200_000.into()));
        assert_eq!(config.amp_offset_db, Some(0));
        assert_eq!(config.calc_mode, Some(CalcMode::Normal));
    }

    #[test]
    fn parse_wsub1g_plus_config() {
        let bytes =
            b"#C2-F:0096000,0090072,-010,-120,0112,0,000,0000050,0960000,0959950,00110,0000,000";
        let config = Config::try_from(bytes.as_ref()).unwrap();
        assert_eq!(config.start.as_hz(), 96_000_000);
        assert_eq!(config.step.as_hz(), 90_072);
        assert_eq!(config.max_amp_dbm, -10);
        assert_eq!(config.min_amp_dbm, -120);
        assert_eq!(config.sweep_points, 112);
        assert!(!config.is_expansion_radio_module_active);
        assert_eq!(config.mode, Mode::SpectrumAnalyzer);
        assert_eq!(config.min_freq.as_hz(), 50_000);
        assert_eq!(config.max_freq.as_hz(), 960_000_000);
        assert_eq!(config.max_span.as_hz(), 959_950_000);
        assert_eq!(config.rbw, Some(110_000.into()));
        assert_eq!(config.amp_offset_db, Some(0));
        assert_eq!(config.calc_mode, Some(CalcMode::Normal));
    }

    #[test]
    fn parse_config_without_rbw_amp_offset_calc_mode() {
        let bytes = b"#C2-F:5249000,0196428,-030,-118,0112,0,000,4850000,6100000,0600000";
        let config = Config::try_from(bytes.as_ref()).unwrap();
        assert_eq!(config.rbw, None);
        assert_eq!(config.amp_offset_db, None);
        assert_eq!(config.calc_mode, None);
    }

    #[test]
    fn fail_to_parse_config_with_incorrect_prefix() {
        let bytes =
            b"#D2-F:0096000,0090072,-010,-120,0112,0,000,0000050,0960000,0959950,00110,0000,000";
        assert!(Config::try_from(bytes.as_ref()).is_err());
    }

    #[test]
    fn fail_to_parse_config_with_invalid_start_freq() {
        let bytes =
            b"#C2-F:XX96000,0090072,-010,-120,0112,0,000,0000050,0960000,0959950,00110,0000,000";
        assert!(Config::try_from(bytes.as_ref()).is_err());
    }
}
