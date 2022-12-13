use super::{Config, ConfigAmpSweep, ConfigCw, ConfigFreqSweep, SetupInfo, Temperature};
use crate::{
    rf_explorer::{ScreenData, SerialNumber},
    SignalGenerator,
};
use nom::error::{Error, ErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    Config(Config),
    ConfigAmpSweep(ConfigAmpSweep),
    ConfigCw(ConfigCw),
    ConfigFreqSweep(ConfigFreqSweep),
    ScreenData(ScreenData),
    SerialNumber(SerialNumber),
    SetupInfo(SetupInfo<SignalGenerator>),
    Temperature(Temperature),
}

impl crate::rf_explorer::Message for Message {
    fn parse(bytes: &[u8]) -> Result<Message, nom::Err<Error<&[u8]>>> {
        if bytes.starts_with(Config::PREFIX) {
            Ok(Message::Config(Config::parse(bytes)?.1))
        } else if bytes.starts_with(ConfigAmpSweep::PREFIX) {
            Ok(Message::ConfigAmpSweep(ConfigAmpSweep::parse(bytes)?.1))
        } else if bytes.starts_with(ConfigCw::PREFIX) {
            Ok(Message::ConfigCw(ConfigCw::parse(bytes)?.1))
        } else if bytes.starts_with(ConfigFreqSweep::PREFIX) {
            Ok(Message::ConfigFreqSweep(ConfigFreqSweep::parse(bytes)?.1))
        } else if bytes.starts_with(ScreenData::PREFIX) {
            Ok(Message::ScreenData(ScreenData::parse(bytes)?.1))
        } else if bytes.starts_with(SerialNumber::PREFIX) {
            Ok(Message::SerialNumber(SerialNumber::parse(bytes)?.1))
        } else if bytes.starts_with(SetupInfo::<SignalGenerator>::PREFIX) {
            Ok(Message::SetupInfo(
                SetupInfo::<SignalGenerator>::parse(bytes)?.1,
            ))
        } else if bytes.starts_with(Temperature::PREFIX) {
            Ok(Message::Temperature(Temperature::parse(bytes)?.1))
        } else {
            Err(nom::Err::Failure(Error::new(bytes, ErrorKind::Fail)))
        }
    }
}
