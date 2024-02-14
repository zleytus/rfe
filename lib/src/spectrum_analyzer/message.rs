use super::{Config, DspMode, InputStage, Model, Sweep, TrackingStatus};
use crate::common::MessageParseError;
use crate::rf_explorer::{ScreenData, SerialNumber, SetupInfo};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Message {
    Config(Config),
    DspMode(DspMode),
    InputStage(InputStage),
    ScreenData(ScreenData),
    SerialNumber(SerialNumber),
    SetupInfo(SetupInfo<Model>),
    Sweep(Sweep),
    TrackingStatus(TrackingStatus),
}

impl<'a> TryFrom<&'a [u8]> for Message {
    type Error = MessageParseError<'a>;

    #[tracing::instrument(ret, err, fields(bytes_as_string = String::from_utf8_lossy(bytes).as_ref()))]
    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.starts_with(Config::PREFIX) {
            Ok(Message::Config(Config::try_from(bytes)?))
        } else if bytes.starts_with(DspMode::PREFIX) {
            Ok(Message::DspMode(DspMode::try_from(bytes)?))
        } else if bytes.starts_with(InputStage::PREFIX) {
            Ok(Message::InputStage(InputStage::try_from(bytes)?))
        } else if bytes.starts_with(ScreenData::PREFIX) {
            Ok(Message::ScreenData(ScreenData::try_from(bytes)?))
        } else if bytes.starts_with(SerialNumber::PREFIX) {
            Ok(Message::SerialNumber(SerialNumber::try_from(bytes)?))
        } else if bytes.starts_with(SetupInfo::<Model>::PREFIX) {
            Ok(Message::SetupInfo(SetupInfo::<Model>::try_from(bytes)?))
        } else if bytes.starts_with(Sweep::STANDARD_PREFIX)
            || bytes.starts_with(Sweep::EXT_PREFIX)
            || bytes.starts_with(Sweep::LARGE_PREFIX)
        {
            Ok(Message::Sweep(Sweep::try_from(bytes)?))
        } else if bytes.starts_with(TrackingStatus::PREFIX) {
            Ok(Message::TrackingStatus(TrackingStatus::try_from(bytes)?))
        } else {
            Err(MessageParseError::UnknownMessageType)
        }
    }
}
