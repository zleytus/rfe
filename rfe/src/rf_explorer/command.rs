use std::borrow::Cow;

use crate::common::BaudRate;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Command {
    RequestConfig,
    RequestSerialNumber,
    EnableLcd,
    DisableLcd,
    EnableDumpScreen,
    DisableDumpScreen,
    Hold,
    SetBaudRate { baud_rate: BaudRate },
    Reboot,
    PowerOff,
}

impl From<Command> for Cow<'static, [u8]> {
    fn from(command: Command) -> Self {
        match command {
            Command::RequestConfig => Cow::Borrowed(&[b'#', 4, b'C', b'0']),
            Command::RequestSerialNumber => Cow::Borrowed(&[b'#', 4, b'C', b'n']),
            Command::EnableLcd => Cow::Borrowed(&[b'#', 4, b'L', b'1']),
            Command::DisableLcd => Cow::Borrowed(&[b'#', 4, b'L', b'0']),
            Command::EnableDumpScreen => Cow::Borrowed(&[b'#', 4, b'D', b'1']),
            Command::DisableDumpScreen => Cow::Borrowed(&[b'#', 4, b'D', b'0']),
            Command::Hold => Cow::Borrowed(&[b'#', 4, b'C', b'H']),
            Command::SetBaudRate { baud_rate } => Cow::Owned(vec![b'#', 4, b'c', baud_rate.code()]),
            Command::Reboot => Cow::Borrowed(&[b'#', 3, b'r']),
            Command::PowerOff => Cow::Borrowed(&[b'#', 3, b'S']),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_correct_size {
        ($command:expr) => {
            let command_bytes = Cow::from($command);
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
        assert_correct_size!(Command::SetBaudRate {
            baud_rate: BaudRate::default()
        });
        assert_correct_size!(Command::Reboot);
        assert_correct_size!(Command::PowerOff);
    }
}
