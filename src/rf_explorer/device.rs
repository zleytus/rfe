use super::serial_port::ConnectionResult;
use super::{ConnectionError, ParseFromBytes, SerialNumber, SerialPortReader};
use serialport::SerialPortInfo;
use std::io::{self, BufRead};
use std::time::{Duration, Instant};

pub trait Device: Sized {
    const COMMAND_RESPONSE_TIMEOUT: Duration = Duration::from_secs(2);
    const READ_SETUP_CONFIG_TIMEOUT: Duration = Duration::from_secs(1);

    type SetupInfo: super::SetupInfo;
    type Config: super::Message;

    fn connect(serial_port_info: &SerialPortInfo) -> ConnectionResult<Self>;

    fn send_bytes(&mut self, bytes: impl AsRef<[u8]>) -> io::Result<()>;

    fn setup_info(&self) -> &Self::SetupInfo;

    fn serial_number(&self) -> Option<SerialNumber>;

    /// Attempts to read the SetupInfo and Config sent by the RF Explorer when we connect to it.
    fn read_setup_and_config(
        serial_port: &mut SerialPortReader,
    ) -> ConnectionResult<(Self::Config, Self::SetupInfo)> {
        let (mut initial_config, mut initial_setup_info) = (None, None);
        let mut message_buf = Vec::new();
        let start_time = Instant::now();

        while start_time.elapsed() < Self::READ_SETUP_CONFIG_TIMEOUT {
            if initial_config.is_some() && initial_setup_info.is_some() {
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

            message_buf.clear();
        }

        match (initial_config, initial_setup_info) {
            (Some(config), Some(setup_info)) => Ok((config, setup_info)),
            _ => Err(ConnectionError::NotAnRfExplorer),
        }
    }
}
