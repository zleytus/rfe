use std::{
    fmt::Debug,
    io::{self, ErrorKind},
    sync::Arc,
    thread,
    time::Duration,
};

use tracing::debug;

use super::{ConnectionError, ConnectionResult, MessageParseError, SerialPort};

pub(crate) trait Device: Debug + Sized {
    const COMMAND_RESPONSE_TIMEOUT: Duration = Duration::from_secs(2);

    type Message: for<'a> TryFrom<&'a [u8], Error = MessageParseError<'a>> + Debug;

    #[tracing::instrument(skip(serial_port), ret, err)]
    fn connect(serial_port: SerialPort) -> ConnectionResult<Arc<Self>> {
        let device = Arc::new(Self::new(serial_port));

        // Read messages from the RF Explorer on a background thread
        Self::start_read_thread(&device);

        if let Err(err) = device.request_device_info() {
            device.stop_reading_messages();
            return Err(err.into());
        }

        if device.wait_for_device_info() {
            // The largest sweep we could receive contains 65,535 (2^16) points
            // To be safe, set the maximum message length to 131,072 (2^17)
            device.serial_port().set_max_message_len(131_072);
            Ok(device)
        } else {
            device.stop_reading_messages();
            Err(ConnectionError::Io(io::ErrorKind::TimedOut.into()))
        }
    }

    fn new(serial_port: SerialPort) -> Self;

    fn start_read_thread(device: &Arc<Self>);

    fn serial_port(&self) -> &SerialPort;

    fn is_reading(&self) -> bool;

    fn firmware_version(&self) -> String;

    fn request_device_info(&self) -> io::Result<()>;

    fn wait_for_device_info(&self) -> bool;

    fn cache_message(&self, message: Self::Message);

    #[tracing::instrument(skip(device))]
    fn read_messages(device: Arc<Self>) {
        debug!("Started reading messages from RF Explorer");
        let mut message_buf = Vec::new();
        while device.is_reading() {
            // Messages from RF Explorers are delimited by \r\n, so we try to read a line from
            // the serial port into the message buffer
            if let Err(error) = device.serial_port().read_line(&mut message_buf) {
                // Time out errors are recoverable so we try to read again
                // Other errors are not recoverable so we break out of the loop
                if error.kind() == ErrorKind::TimedOut {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                } else {
                    break;
                }
            }

            match find_message_in_buf(&message_buf) {
                Ok(message) => {
                    device.cache_message(message);
                    message_buf.clear()
                }
                Err(MessageParseError::Incomplete) => (),
                Err(_) => message_buf.clear(),
            }

            thread::sleep(Duration::from_millis(10));
        }
        debug!("Stopped reading messages from RF Explorer");
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
