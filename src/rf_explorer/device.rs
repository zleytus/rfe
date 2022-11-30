use super::serial_port::ConnectionResult;
use super::{ConnectionError, ParseFromBytes, SerialNumber, SerialPortReader};
use serialport::SerialPortInfo;
use std::io::{self, BufRead};
use std::time::{Duration, Instant};

pub trait Device: Sized {
    const COMMAND_RESPONSE_TIMEOUT: Duration = Duration::from_secs(2);
    const READ_SETUP_CONFIG_TIMEOUT: Duration = Duration::from_secs(1);

    type SetupInfo: super::Message;
    type Config: super::Message;

    fn connect(serial_port_info: &SerialPortInfo) -> ConnectionResult<Self>;

    fn send_bytes(&mut self, bytes: impl AsRef<[u8]>) -> io::Result<()>;

    fn port_name(&self) -> &str;

    fn setup_info(&self) -> &Self::SetupInfo;

    fn serial_number(&self) -> SerialNumber;

    /// Attempts to read the `SetupInfo`, `Config`, and `SerialNumber` sent by the RF Explorer when we connect to it.
    fn read_initial_messages(
        serial_port: &mut SerialPortReader,
    ) -> ConnectionResult<(Self::Config, Self::SetupInfo, SerialNumber)> {
        let (mut initial_config, mut initial_setup_info, mut initial_serial_number) =
            (None, None, None);
        let mut message_buf = Vec::new();
        let start_time = Instant::now();

        while start_time.elapsed() < Self::READ_SETUP_CONFIG_TIMEOUT {
            if initial_config.is_some()
                && initial_setup_info.is_some()
                && initial_serial_number.is_some()
            {
                break;
            }

            serial_port.read_until(b'\n', &mut message_buf)?;

            if initial_config.is_none() {
                if let Ok((_, config)) = Self::Config::parse_from_bytes(message_buf.as_slice()) {
                    initial_config = Some(config);
                    message_buf.clear();
                    continue;
                }
            }

            if initial_setup_info.is_none() {
                if let Ok((_, setup_info)) =
                    Self::SetupInfo::parse_from_bytes(message_buf.as_slice())
                {
                    initial_setup_info = Some(setup_info);
                    message_buf.clear();
                    continue;
                }
            }

            if initial_serial_number.is_none() {
                if let Ok((_, serial_number)) =
                    SerialNumber::parse_from_bytes(message_buf.as_slice())
                {
                    initial_serial_number = Some(serial_number);
                    message_buf.clear();
                    continue;
                }
            }

            message_buf.clear();
        }

        if let (Some(config), Some(setup_info), Some(serial_number)) =
            (initial_config, initial_setup_info, initial_serial_number)
        {
            Ok((config, setup_info, serial_number))
        } else {
            Err(ConnectionError::NotAnRfExplorer)
        }
    }
}
