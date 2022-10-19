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
    const PREFIX: &'static [u8] = b"#C3-M:";
}

#[cfg(test)]
mod tests {
    use crate::rf_explorer::ParseFromBytes;
    use crate::Model;

    #[test]
    fn accept_rfe_gen_setup() {
        let setup = super::SetupInfo::parse_from_bytes(b"#C3-M:060,255,01.15\r\n".as_ref())
            .unwrap()
            .1;
        assert_eq!(setup.main_model, Model::RfeGen);
        assert_eq!(setup.exp_model, Model::None);
        assert_eq!(setup.fw_version, "01.15");
    }
}
