use super::{
    Config, ConfigAmpSweep, ConfigAmpSweepExp, ConfigCw, ConfigCwExp, ConfigExp, ConfigFreqSweep,
    ConfigFreqSweepExp, Model, Temperature,
};
use crate::common::{MessageParseError, ScreenData, SerialNumber, SetupInfo};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    Config(Config),
    ConfigAmpSweep(ConfigAmpSweep),
    ConfigCw(ConfigCw),
    ConfigFreqSweep(ConfigFreqSweep),
    ScreenData(ScreenData),
    SerialNumber(SerialNumber),
    SetupInfo(SetupInfo<Model>),
    Temperature(Temperature),
}

impl<'a> TryFrom<&'a [u8]> for Message {
    type Error = MessageParseError<'a>;

    #[tracing::instrument(ret, err, fields(bytes_as_string = String::from_utf8_lossy(bytes).as_ref()))]
    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.starts_with(Config::PREFIX) {
            Ok(Message::Config(Config::try_from(bytes)?))
        } else if bytes.starts_with(ConfigAmpSweep::PREFIX) {
            Ok(Message::ConfigAmpSweep(ConfigAmpSweep::try_from(bytes)?))
        } else if bytes.starts_with(ConfigCw::PREFIX) {
            Ok(Message::ConfigCw(ConfigCw::try_from(bytes)?))
        } else if bytes.starts_with(ConfigFreqSweep::PREFIX) {
            Ok(Message::ConfigFreqSweep(ConfigFreqSweep::try_from(bytes)?))
        } else if bytes.starts_with(ScreenData::PREFIX) {
            Ok(Message::ScreenData(ScreenData::try_from(bytes)?))
        } else if bytes.starts_with(SerialNumber::PREFIX) {
            Ok(Message::SerialNumber(SerialNumber::try_from(bytes)?))
        } else if bytes.starts_with(SetupInfo::<Model>::PREFIX) {
            Ok(Message::SetupInfo(SetupInfo::<Model>::try_from(bytes)?))
        } else if bytes.starts_with(Temperature::PREFIX) {
            Ok(Message::Temperature(Temperature::try_from(bytes)?))
        } else {
            Err(crate::common::MessageParseError::UnknownMessageType)
        }
    }
}
