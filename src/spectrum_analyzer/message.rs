use super::{
    sweep::{SweepDataExt, SweepDataLarge, SweepDataStandard},
    Config, DspMode, InputStage, Model, Sweep, TrackingStatus,
};
use crate::common::{MessageParseError, ScreenData, SerialNumber, SetupInfo};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Config(Config),
    DspMode(DspMode),
    InputStage(InputStage),
    ScreenData(ScreenData),
    SerialNumber(SerialNumber),
    SetupInfo(SetupInfo<Model>),
    Sweep(Sweep),
    TrackingStatus(TrackingStatus),
}

impl crate::common::Message for Message {
    fn parse(bytes: &[u8]) -> Result<Self, MessageParseError> {
        if bytes.starts_with(Config::PREFIX) {
            Ok(Message::Config(Config::parse(bytes)?.1))
        } else if bytes.starts_with(DspMode::PREFIX) {
            Ok(Message::DspMode(DspMode::parse(bytes)?.1))
        } else if bytes.starts_with(InputStage::PREFIX) {
            Ok(Message::InputStage(InputStage::parse(bytes)?.1))
        } else if bytes.starts_with(ScreenData::PREFIX) {
            Ok(Message::ScreenData(ScreenData::parse(bytes)?.1))
        } else if bytes.starts_with(SerialNumber::PREFIX) {
            Ok(Message::SerialNumber(SerialNumber::parse(bytes)?.1))
        } else if bytes.starts_with(SetupInfo::<Model>::PREFIX) {
            Ok(Message::SetupInfo(SetupInfo::<Model>::parse(bytes)?.1))
        } else if bytes.starts_with(SweepDataStandard::PREFIX) {
            Ok(Message::Sweep(Sweep::Standard(
                SweepDataStandard::parse(bytes)?.1,
            )))
        } else if bytes.starts_with(SweepDataExt::PREFIX) {
            Ok(Message::Sweep(Sweep::Ext(SweepDataExt::parse(bytes)?.1)))
        } else if bytes.starts_with(SweepDataLarge::PREFIX) {
            Ok(Message::Sweep(Sweep::Large(
                SweepDataLarge::parse(bytes)?.1,
            )))
        } else if bytes.starts_with(TrackingStatus::PREFIX) {
            Ok(Message::TrackingStatus(TrackingStatus::parse(bytes)?.1))
        } else {
            Err(MessageParseError::UnknownMessageType)
        }
    }
}
