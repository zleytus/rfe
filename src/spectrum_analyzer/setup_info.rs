use crate::{common::SetupInfo, SpectrumAnalyzer};

impl SetupInfo<SpectrumAnalyzer> {
    pub const PREFIX: &'static [u8] = b"#C2-M:";

    pub(crate) fn parse(bytes: &[u8]) -> nom::IResult<&[u8], Self> {
        Self::parse_from_bytes_with_prefix(bytes, Self::PREFIX)
    }
}

impl Clone for SetupInfo<SpectrumAnalyzer> {
    fn clone(&self) -> Self {
        Self {
            main_module_model: self.main_module_model,
            expansion_module_model: self.expansion_module_model,
            firmware_version: self.firmware_version.clone(),
            marker: self.marker,
        }
    }
}

impl PartialEq for SetupInfo<SpectrumAnalyzer> {
    fn eq(&self, other: &Self) -> bool {
        self.main_module_model == other.main_module_model
            && self.expansion_module_model == other.expansion_module_model
            && self.firmware_version == other.firmware_version
    }
}

impl Eq for SetupInfo<SpectrumAnalyzer> {}

#[cfg(test)]
mod tests {
    use crate::common::SetupInfo;
    use crate::{Model, SpectrumAnalyzer};

    #[test]
    fn accept_wsub1g_setup() {
        let setup = SetupInfo::<SpectrumAnalyzer>::parse(b"#C2-M:003,255,XX.XXXX".as_ref())
            .unwrap()
            .1;
        assert_eq!(setup.main_module_model, Model::RfeWSub1G);
        assert_eq!(setup.expansion_module_model, Model::None);
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_24g_setup() {
        let setup = SetupInfo::<SpectrumAnalyzer>::parse(b"#C2-M:004,255,XX.XXXX".as_ref())
            .unwrap()
            .1;
        assert_eq!(setup.main_module_model, Model::Rfe24G);
        assert_eq!(setup.expansion_module_model, Model::None);
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_ism_combo_setup() {
        let setup = SetupInfo::<SpectrumAnalyzer>::parse(b"#C2-M:003,004,XX.XXXX".as_ref())
            .unwrap()
            .1;
        assert_eq!(setup.main_module_model, Model::RfeWSub1G);
        assert_eq!(setup.expansion_module_model, Model::Rfe24G);
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_3g_combo_setup() {
        let setup = SetupInfo::<SpectrumAnalyzer>::parse(b"#C2-M:003,005,XX.XXXX".as_ref())
            .unwrap()
            .1;
        assert_eq!(setup.main_module_model, Model::RfeWSub1G);
        assert_eq!(setup.expansion_module_model, Model::RfeWSub3G);
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_6g_combo_setup() {
        let setup = SetupInfo::<SpectrumAnalyzer>::parse(b"#C2-M:006,005,XX.XXXX".as_ref())
            .unwrap()
            .1;
        assert_eq!(setup.main_module_model, Model::Rfe6G);
        assert_eq!(setup.expansion_module_model, Model::RfeWSub3G);
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_wsub1g_plus_setup() {
        let setup = SetupInfo::<SpectrumAnalyzer>::parse(b"#C2-M:010,255,XX.XXXX".as_ref())
            .unwrap()
            .1;
        assert_eq!(setup.main_module_model, Model::RfeWSub1GPlus);
        assert_eq!(setup.expansion_module_model, Model::None);
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_ism_combo_plus_setup() {
        let setup = SetupInfo::<SpectrumAnalyzer>::parse(b"#C2-M:010,012,XX.XXXX".as_ref())
            .unwrap()
            .1;
        assert_eq!(setup.main_module_model, Model::RfeWSub1GPlus);
        assert_eq!(setup.expansion_module_model, Model::Rfe24GPlus);
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_4g_combo_plus_setup() {
        let setup = SetupInfo::<SpectrumAnalyzer>::parse(b"#C2-M:010,013,XX.XXXX".as_ref())
            .unwrap()
            .1;
        assert_eq!(setup.main_module_model, Model::RfeWSub1GPlus);
        assert_eq!(setup.expansion_module_model, Model::Rfe4GPlus);
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_6g_combo_plus_setup() {
        let setup = SetupInfo::<SpectrumAnalyzer>::parse(b"#C2-M:010,014,XX.XXXX".as_ref())
            .unwrap()
            .1;
        assert_eq!(setup.main_module_model, Model::RfeWSub1GPlus);
        assert_eq!(setup.expansion_module_model, Model::Rfe6GPlus);
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn reject_setup_without_main_model() {
        assert!(SetupInfo::<SpectrumAnalyzer>::parse(b"#C2-M:255,005,01.12B26".as_ref()).is_err());
    }

    #[test]
    fn accept_setup_without_expansion_model() {
        assert!(SetupInfo::<SpectrumAnalyzer>::parse(b"#C2-M:006,255,01.12B26".as_ref()).is_ok());
    }

    #[test]
    fn reject_setup_without_firmware_version() {
        assert!(SetupInfo::<SpectrumAnalyzer>::parse(b"#C2-M:006,005".as_ref()).is_err());
    }

    #[test]
    fn reject_setup_with_incorrect_prefix() {
        assert!(SetupInfo::<SpectrumAnalyzer>::parse(b"$C2-M:006,005,01.12B26".as_ref()).is_err());
    }
}
