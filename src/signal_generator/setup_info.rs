use super::Model;
use crate::common::SetupInfo;

impl SetupInfo<Model> {
    pub const PREFIX: &'static [u8] = b"#C3-M:";

    pub(crate) fn parse(bytes: &[u8]) -> nom::IResult<&[u8], Self> {
        SetupInfo::parse_with_prefix(bytes, Self::PREFIX)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{RadioModule, SetupInfo};
    use crate::signal_generator::Model;

    #[test]
    fn accept_rfe_gen_setup() {
        let setup = SetupInfo::<Model>::parse(b"#C3-M:060,255,01.15\r\n".as_ref())
            .unwrap()
            .1;
        assert_eq!(
            setup.main_radio_module,
            RadioModule::Main {
                model: Model::Rfe6Gen
            }
        );
        assert_eq!(setup.expansion_radio_module, None);
        assert_eq!(setup.firmware_version, "01.15");
    }

    #[test]
    fn accept_rfe_gen_combo_setup() {
        let setup = SetupInfo::<Model>::parse(b"#C3-M:060,061,01.15\r\n".as_ref())
            .unwrap()
            .1;
        assert_eq!(
            setup.main_radio_module,
            RadioModule::Main {
                model: Model::Rfe6Gen
            }
        );
        assert_eq!(
            setup.expansion_radio_module,
            Some(RadioModule::Expansion {
                model: Model::Rfe6GenExpansion
            })
        );
        assert_eq!(setup.firmware_version, "01.15");
    }
}
