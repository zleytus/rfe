use crate::{common::SetupInfo, SignalGenerator};

impl SetupInfo<SignalGenerator> {
    pub const PREFIX: &'static [u8] = b"#C3-M:";

    pub(crate) fn parse(bytes: &[u8]) -> nom::IResult<&[u8], Self> {
        SetupInfo::parse_with_prefix(bytes, Self::PREFIX)
    }
}

impl Clone for SetupInfo<SignalGenerator> {
    fn clone(&self) -> Self {
        Self {
            main_module_model: self.main_module_model,
            expansion_module_model: self.expansion_module_model,
            firmware_version: self.firmware_version.clone(),
            marker: self.marker,
        }
    }
}

impl PartialEq for SetupInfo<SignalGenerator> {
    fn eq(&self, other: &Self) -> bool {
        self.main_module_model == other.main_module_model
            && self.expansion_module_model == other.expansion_module_model
            && self.firmware_version == other.firmware_version
    }
}

impl Eq for SetupInfo<SignalGenerator> {}

#[cfg(test)]
mod tests {
    use crate::common::SetupInfo;
    use crate::{Model, SignalGenerator};

    #[test]
    fn accept_rfe_gen_setup() {
        let setup = SetupInfo::<SignalGenerator>::parse(b"#C3-M:060,255,01.15\r\n".as_ref())
            .unwrap()
            .1;
        assert_eq!(setup.main_module_model, Model::RfeGen);
        assert_eq!(setup.expansion_module_model, None);
        assert_eq!(setup.firmware_version, "01.15");
    }
}
