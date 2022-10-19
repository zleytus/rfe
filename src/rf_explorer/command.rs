#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Command {
    RequestConfig,
    RequestSerialNumber,
    EnableLcd,
    DisableLcd,
    EnableDumpScreen,
    DisableDumpScreen,
    Hold,
    Reboot,
    PowerOff,
}

impl AsRef<[u8]> for Command {
    fn as_ref(&self) -> &[u8] {
        match self {
            Command::RequestConfig => &[b'#', 4, b'C', b'0'],
            Command::RequestSerialNumber => &[b'#', 4, b'C', b'n'],
            Command::EnableLcd => &[b'#', 4, b'L', b'1'],
            Command::DisableLcd => &[b'#', 4, b'L', b'0'],
            Command::EnableDumpScreen => &[b'#', 4, b'D', b'1'],
            Command::DisableDumpScreen => &[b'#', 4, b'D', b'0'],
            Command::Hold => &[b'#', 4, b'C', b'H'],
            Command::Reboot => &[b'#', 3, b'r'],
            Command::PowerOff => &[b'#', 3, b'S'],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_correct_size {
        ($command:expr) => {
            let command_bytes = $command.as_ref();
            assert_eq!(command_bytes[1], command_bytes.len() as u8);
        };
    }

    #[test]
    fn correct_command_size_fields() {
        assert_correct_size!(Command::RequestConfig);
        assert_correct_size!(Command::RequestSerialNumber);
        assert_correct_size!(Command::EnableLcd);
        assert_correct_size!(Command::DisableLcd);
        assert_correct_size!(Command::EnableDumpScreen);
        assert_correct_size!(Command::DisableDumpScreen);
        assert_correct_size!(Command::Hold);
        assert_correct_size!(Command::Reboot);
        assert_correct_size!(Command::PowerOff);
    }
}
