use crate::rf_explorer::{Message, Model};
use std::str;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SetupInfo {
    main_model: Model,
    exp_model: Model,
    fw_version: String,
}

impl crate::rf_explorer::SetupInfo for SetupInfo {
    fn new(main_model: Model, exp_model: Model, fw_version: String) -> Self {
        SetupInfo {
            main_model,
            exp_model,
            fw_version,
        }
    }

    fn main_module_model(&self) -> Model {
        self.main_model
    }

    fn expansion_module_model(&self) -> Model {
        self.exp_model
    }

    fn firmware_version(&self) -> &str {
        &self.fw_version
    }
}

impl Message for SetupInfo {
    const PREFIX: &'static [u8] = b"#C2-M:";
}

#[cfg(test)]
mod tests {
    use crate::rf_explorer::{ParseFromBytes, SetupInfo};
    use crate::Model;

    #[test]
    fn accept_wsub1g_setup() {
        let setup: crate::spectrum_analyzer::SetupInfo =
            super::SetupInfo::parse_from_bytes(b"#C2-M:003,255,XX.XXXX".as_ref())
                .unwrap()
                .1;
        assert_eq!(setup.main_module_model(), Model::RfeWSub1G);
        assert_eq!(setup.expansion_module_model(), Model::None);
        assert_eq!(setup.firmware_version(), "XX.XXXX");
    }

    #[test]
    fn accept_24g_setup() {
        let setup: crate::spectrum_analyzer::SetupInfo =
            super::SetupInfo::parse_from_bytes(b"#C2-M:004,255,XX.XXXX".as_ref())
                .unwrap()
                .1;
        assert_eq!(setup.main_module_model(), Model::Rfe24G);
        assert_eq!(setup.expansion_module_model(), Model::None);
        assert_eq!(setup.firmware_version(), "XX.XXXX");
    }

    #[test]
    fn accept_ism_combo_setup() {
        let setup: crate::spectrum_analyzer::SetupInfo =
            super::SetupInfo::parse_from_bytes(b"#C2-M:003,004,XX.XXXX".as_ref())
                .unwrap()
                .1;
        assert_eq!(setup.main_module_model(), Model::RfeWSub1G);
        assert_eq!(setup.expansion_module_model(), Model::Rfe24G);
        assert_eq!(setup.firmware_version(), "XX.XXXX");
    }

    #[test]
    fn accept_3g_combo_setup() {
        let setup: crate::spectrum_analyzer::SetupInfo =
            super::SetupInfo::parse_from_bytes(b"#C2-M:003,005,XX.XXXX".as_ref())
                .unwrap()
                .1;
        assert_eq!(setup.main_module_model(), Model::RfeWSub1G);
        assert_eq!(setup.expansion_module_model(), Model::RfeWSub3G);
        assert_eq!(setup.firmware_version(), "XX.XXXX");
    }

    #[test]
    fn accept_6g_combo_setup() {
        let setup: crate::spectrum_analyzer::SetupInfo =
            super::SetupInfo::parse_from_bytes(b"#C2-M:006,005,XX.XXXX".as_ref())
                .unwrap()
                .1;
        assert_eq!(setup.main_module_model(), Model::Rfe6G);
        assert_eq!(setup.expansion_module_model(), Model::RfeWSub3G);
        assert_eq!(setup.firmware_version(), "XX.XXXX");
    }

    #[test]
    fn accept_wsub1g_plus_setup() {
        let setup: crate::spectrum_analyzer::SetupInfo =
            super::SetupInfo::parse_from_bytes(b"#C2-M:010,255,XX.XXXX".as_ref())
                .unwrap()
                .1;
        assert_eq!(setup.main_module_model(), Model::RfeWSub1GPlus);
        assert_eq!(setup.expansion_module_model(), Model::None);
        assert_eq!(setup.firmware_version(), "XX.XXXX");
    }

    #[test]
    fn accept_ism_combo_plus_setup() {
        let setup: crate::spectrum_analyzer::SetupInfo =
            super::SetupInfo::parse_from_bytes(b"#C2-M:010,012,XX.XXXX".as_ref())
                .unwrap()
                .1;
        assert_eq!(setup.main_module_model(), Model::RfeWSub1GPlus);
        assert_eq!(setup.expansion_module_model(), Model::Rfe24GPlus);
        assert_eq!(setup.firmware_version(), "XX.XXXX");
    }

    #[test]
    fn accept_4g_combo_plus_setup() {
        let setup: crate::spectrum_analyzer::SetupInfo =
            super::SetupInfo::parse_from_bytes(b"#C2-M:010,013,XX.XXXX".as_ref())
                .unwrap()
                .1;
        assert_eq!(setup.main_module_model(), Model::RfeWSub1GPlus);
        assert_eq!(setup.expansion_module_model(), Model::Rfe4GPlus);
        assert_eq!(setup.firmware_version(), "XX.XXXX");
    }

    #[test]
    fn accept_6g_combo_plus_setup() {
        let setup: crate::spectrum_analyzer::SetupInfo =
            super::SetupInfo::parse_from_bytes(b"#C2-M:010,014,XX.XXXX".as_ref())
                .unwrap()
                .1;
        assert_eq!(setup.main_module_model(), Model::RfeWSub1GPlus);
        assert_eq!(setup.expansion_module_model(), Model::Rfe6GPlus);
        assert_eq!(setup.firmware_version(), "XX.XXXX");
    }

    #[test]
    fn reject_setup_without_main_model() {
        assert!(crate::spectrum_analyzer::SetupInfo::parse_from_bytes(
            b"#C2-M:255,005,01.12B26".as_ref()
        )
        .is_err());
    }

    #[test]
    fn accept_setup_without_expansion_model() {
        assert!(crate::spectrum_analyzer::SetupInfo::parse_from_bytes(
            b"#C2-M:006,255,01.12B26".as_ref()
        )
        .is_ok());
    }

    #[test]
    fn reject_setup_without_firmware_version() {
        assert!(
            crate::spectrum_analyzer::SetupInfo::parse_from_bytes(b"#C2-M:006,005".as_ref())
                .is_err()
        );
    }

    #[test]
    fn reject_setup_with_incorrect_prefix() {
        assert!(crate::spectrum_analyzer::SetupInfo::parse_from_bytes(
            b"$C2-M:006,005,01.12B26".as_ref()
        )
        .is_err());
    }
}
