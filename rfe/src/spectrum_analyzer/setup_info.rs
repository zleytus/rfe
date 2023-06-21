use super::Model;
use crate::common::MessageParseError;
use crate::rf_explorer::SetupInfo;

impl SetupInfo<Model> {
    pub(crate) const PREFIX: &'static [u8] = b"#C2-M:";
}

impl<'a> TryFrom<&'a [u8]> for SetupInfo<Model> {
    type Error = MessageParseError<'a>;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        Self::try_from_with_prefix(bytes, Self::PREFIX)
    }
}

#[cfg(test)]
mod tests {
    use super::Model;
    use crate::rf_explorer::{RadioModule, SetupInfo};

    #[test]
    fn accept_wsub1g_setup() {
        let setup = SetupInfo::<Model>::try_from(b"#C2-M:003,255,XX.XXXX".as_ref()).unwrap();
        assert_eq!(
            setup.main_radio_module,
            RadioModule::Main {
                model: Model::RfeWSub1G
            }
        );
        assert_eq!(setup.expansion_radio_module, None);
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_24g_setup() {
        let setup = SetupInfo::<Model>::try_from(b"#C2-M:004,255,XX.XXXX".as_ref()).unwrap();
        assert_eq!(
            setup.main_radio_module,
            RadioModule::Main {
                model: Model::Rfe24G
            }
        );
        assert_eq!(setup.expansion_radio_module, None);
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_ism_combo_setup() {
        let setup = SetupInfo::<Model>::try_from(b"#C2-M:003,004,XX.XXXX".as_ref()).unwrap();
        assert_eq!(
            setup.main_radio_module,
            RadioModule::Main {
                model: Model::RfeWSub1G
            }
        );
        assert_eq!(
            setup.expansion_radio_module,
            Some(RadioModule::Expansion {
                model: Model::Rfe24G
            })
        );
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_3g_combo_setup() {
        let setup = SetupInfo::<Model>::try_from(b"#C2-M:003,005,XX.XXXX".as_ref()).unwrap();
        assert_eq!(
            setup.main_radio_module,
            RadioModule::Main {
                model: Model::RfeWSub1G
            }
        );
        assert_eq!(
            setup.expansion_radio_module,
            Some(RadioModule::Expansion {
                model: Model::RfeWSub3G
            })
        );
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_6g_combo_setup() {
        let setup = SetupInfo::<Model>::try_from(b"#C2-M:006,005,XX.XXXX".as_ref()).unwrap();
        assert_eq!(
            setup.main_radio_module,
            RadioModule::Main {
                model: Model::Rfe6G
            }
        );
        assert_eq!(
            setup.expansion_radio_module,
            Some(RadioModule::Expansion {
                model: Model::RfeWSub3G
            })
        );
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_wsub1g_plus_setup() {
        let setup = SetupInfo::<Model>::try_from(b"#C2-M:010,255,XX.XXXX".as_ref()).unwrap();
        assert_eq!(
            setup.main_radio_module,
            RadioModule::Main {
                model: Model::RfeWSub1GPlus
            }
        );
        assert_eq!(setup.expansion_radio_module, None);
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_ism_combo_plus_setup() {
        let setup = SetupInfo::<Model>::try_from(b"#C2-M:010,012,XX.XXXX".as_ref()).unwrap();
        assert_eq!(
            setup.main_radio_module,
            RadioModule::Main {
                model: Model::RfeWSub1GPlus
            }
        );
        assert_eq!(
            setup.expansion_radio_module,
            Some(RadioModule::Expansion {
                model: Model::Rfe24GPlus
            })
        );
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_4g_combo_plus_setup() {
        let setup = SetupInfo::<Model>::try_from(b"#C2-M:010,013,XX.XXXX".as_ref()).unwrap();
        assert_eq!(
            setup.main_radio_module,
            RadioModule::Main {
                model: Model::RfeWSub1GPlus
            }
        );
        assert_eq!(
            setup.expansion_radio_module,
            Some(RadioModule::Expansion {
                model: Model::Rfe4GPlus
            })
        );
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn accept_6g_combo_plus_setup() {
        let setup = SetupInfo::<Model>::try_from(b"#C2-M:010,014,XX.XXXX".as_ref()).unwrap();
        assert_eq!(
            setup.main_radio_module,
            RadioModule::Main {
                model: Model::RfeWSub1GPlus
            }
        );
        assert_eq!(
            setup.expansion_radio_module,
            Some(RadioModule::Expansion {
                model: Model::Rfe6GPlus
            })
        );
        assert_eq!(setup.firmware_version, "XX.XXXX");
    }

    #[test]
    fn reject_setup_without_main_model() {
        assert!(SetupInfo::<Model>::try_from(b"#C2-M:255,005,01.12B26".as_ref()).is_err());
    }

    #[test]
    fn accept_setup_without_expansion_model() {
        assert!(SetupInfo::<Model>::try_from(b"#C2-M:006,255,01.12B26".as_ref()).is_ok());
    }

    #[test]
    fn reject_setup_without_firmware_version() {
        assert!(SetupInfo::<Model>::try_from(b"#C2-M:006,005".as_ref()).is_err());
    }

    #[test]
    fn reject_setup_with_incorrect_prefix() {
        assert!(SetupInfo::<Model>::try_from(b"$C2-M:006,005,01.12B26".as_ref()).is_err());
    }
}
