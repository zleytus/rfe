use std::{
    fmt::Debug,
    io::{self, ErrorKind},
    sync::Arc,
    thread,
    time::Duration,
};

pub trait Device: Sized + Send + Sync {
    const COMMAND_RESPONSE_TIMEOUT: Duration = Duration::from_secs(2);
    const RECEIVE_FIRST_CONFIG_TIMEOUT: Duration = Duration::from_secs(1);
    const EEOT_BYTES: [u8; 5] = [255, 254, 255, 254, 0];

use super::{ConnectionResult, MessageParseError, SerialNumber, SerialPort};

    fn connect(serial_port_info: &SerialPortInfo) -> ConnectionResult<Arc<Self>>;

    fn send_bytes(&self, bytes: impl AsRef<[u8]>) -> io::Result<()>;

    fn connect(serial_port: SerialPort) -> ConnectionResult<Arc<Self>>;

    fn serial_port(&self) -> &SerialPort;

    fn is_reading(&self) -> bool;

    fn firmware_version(&self) -> String;

    fn serial_number(&self) -> SerialNumber;

    fn spawn_read_thread(device: Arc<Self>) -> JoinHandle<()>
    where
        Self: 'static,
    {
        thread::spawn(move || {
            let mut message_buf = Vec::new();
            while device.is_reading() {
                let read_line_result = device.read_line(&mut message_buf);

                // Time out errors are recoverable so we should try to read again
                // Other errors are not recoverable and we should exit the thread
                match read_line_result {
                    Ok(bytes_read) => trace!("Read {} bytes", bytes_read),
                    Err(e) if e.kind() == ErrorKind::TimedOut => {
                        warn!("Read timeout occurred. Attempting to read again.");
                        continue;
                    }
                    Err(e) => {
                        error!("Unrecoverable read error occured: {:?}", e.kind());
                        break;
                    }
                }

                match Self::Message::parse(&message_buf) {
                    Ok(message) => {
                        device.process_message(message);
                        message_buf.clear();
                    }
                    Err(MessageParseError::Incomplete) => {
                        // Check for Early-End-of-Transmission (EEOT) byte sequences
                        while let Some(eeot_index) = message_buf
                            .windows(Self::EEOT_BYTES.len())
                            .position(|window| window == Self::EEOT_BYTES)
                        {
                            warn!("Found partial message with EEOT byte sequence. Removing partial message from message buffer.");
                            message_buf.drain(0..eeot_index + Self::EEOT_BYTES.len());

                            // Try to parse again after removing the EEOT bytes
                            match Self::Message::parse(&message_buf) {
                                Ok(message) => {
                                    device.process_message(message);
                                    message_buf.clear();
                                    break;
                                }
                                Err(MessageParseError::Incomplete) => {
                                    continue;
                                }
                                _ => {
                                    message_buf.clear();
                                    break;
                                }
                            }
                        }
                    }
                    _ => message_buf.clear(),
                }
            }
        })
    }

    fn stop_reading_messages(&self);
}

fn find_message_in_buf<M>(message_buf: &[u8]) -> Result<M, MessageParseError>
where
    M: for<'a> TryFrom<&'a [u8], Error = MessageParseError<'a>>,
{
    M::try_from(message_buf).or_else(|e| match e {
        MessageParseError::Truncated {
            remainder: Some(remaining_bytes),
        } => find_message_in_buf(remaining_bytes),
        error => Err(error),
    })
}
