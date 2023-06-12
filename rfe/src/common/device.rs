use std::{
    borrow::Cow,
    fmt::Debug,
    io::{self, ErrorKind},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use tracing::debug;

use super::{serial_port, ConnectionError, ConnectionResult, MessageParseError, SerialPort};

#[derive(Debug)]
pub struct Device<M: MessageContainer + 'static> {
    serial_port: Arc<SerialPort>,
    is_reading: Arc<AtomicBool>,
    read_thread_handle: Option<JoinHandle<()>>,
    messages: Arc<M>,
}

impl<M: MessageContainer> Device<M> {
    fn connect_internal(
        serial_port: SerialPort,
        device_init_command: impl AsRef<[u8]> + Debug,
    ) -> ConnectionResult<Self> {
        let mut device = Self {
            serial_port: Arc::new(serial_port),
            is_reading: Arc::new(AtomicBool::new(true)),
            read_thread_handle: None,
            messages: Arc::new(M::default()),
        };

        // Read messages from the device on a background thread
        // let device_clone = device.clone();
        let messages = device.messages.clone();
        let serial_port = device.serial_port.clone();
        let is_reading = device.is_reading.clone();
        device.read_thread_handle = Some(thread::spawn(move || {
            Self::read_messages(serial_port, messages, is_reading)
        }));

        if let Err(err) = device.serial_port.send_bytes(device_init_command) {
            device.stop_reading_messages();
            return Err(err.into());
        }

        if device.messages.wait_for_device_info() {
            // The largest sweep we could receive contains 65,535 (2^16) points
            // To be safe, set the maximum message length to 131,072 (2^17)
            device.serial_port.set_max_message_len(131_072);
            Ok(device)
        } else {
            device.stop_reading_messages();
            Err(ConnectionError::Io(io::ErrorKind::TimedOut.into()))
        }
    }

    pub fn connect(device_init_command: impl AsRef<[u8]>) -> Option<Self> {
        // For every Silabs CP210X port, we first try to connect using the RF Explorer's fast
        // default baud rate (500 kbps) and then try to connect using its slow default baud rate
        // (2.4 kbps)
        serial_port::silabs_cp210x_ports()
            .flat_map(|port_info| {
                [
                    (port_info.clone(), serial_port::FAST_BAUD_RATE),
                    (port_info, serial_port::SLOW_BAUD_RATE),
                ]
            })
            .find_map(|(port_info, baud_rate)| {
                let serial_port = SerialPort::open(&port_info, baud_rate).ok()?;
                Self::connect_internal(serial_port, device_init_command.as_ref()).ok()
            })
    }

    pub fn connect_with_name_and_baud_rate(
        name: &str,
        baud_rate: u32,
        device_init_command: impl AsRef<[u8]>,
    ) -> ConnectionResult<Self> {
        let serial_port = SerialPort::open_with_name(name, baud_rate)?;
        Self::connect_internal(serial_port, device_init_command.as_ref())
    }

    pub fn connect_all(init_command: impl AsRef<[u8]>) -> Vec<Self> {
        serial_port::silabs_cp210x_ports()
            .flat_map(|port_info| {
                [
                    (port_info.clone(), serial_port::FAST_BAUD_RATE),
                    (port_info, serial_port::SLOW_BAUD_RATE),
                ]
            })
            .filter_map(|(port_info, baud_rate)| {
                println!("{port_info:#?}, {baud_rate}");
                let serial_port = SerialPort::open(&port_info, baud_rate).ok()?;
                Self::connect_internal(serial_port, init_command.as_ref()).ok()
            })
            .collect()
    }

    fn read_messages(serial_port: Arc<SerialPort>, messages: Arc<M>, is_reading: Arc<AtomicBool>) {
        debug!("Started reading messages from device");
        let mut message_buf = Vec::new();
        while is_reading.load(Ordering::Relaxed) {
            // Messages from devices are delimited by \r\n, so we try to read a line from
            // the serial port into the message buffer
            if let Err(error) = serial_port.read_line(&mut message_buf) {
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
                    messages.cache_message(message);
                    message_buf.clear()
                }
                Err(MessageParseError::Incomplete) => (),
                Err(_) => message_buf.clear(),
            }

            thread::sleep(Duration::from_millis(10));
        }
        debug!("Stopped reading messages from device");
    }

    pub fn messages(&self) -> &M {
        &self.messages
    }

    pub(crate) fn serial_port(&self) -> &SerialPort {
        &self.serial_port
    }

    pub fn send_bytes(&self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
        self.serial_port.send_bytes(bytes.as_ref())
    }

    pub fn send_command(&self, command: impl Into<Cow<'static, [u8]>>) -> io::Result<()> {
        self.serial_port.send_command(command.into())
    }

    pub fn port_name(&self) -> &str {
        &self.serial_port.port_info().port_name
    }

    pub fn baud_rate(&self) -> io::Result<u32> {
        self.serial_port.baud_rate()
    }

    fn stop_reading_messages(&mut self) {
        self.is_reading.store(false, Ordering::Relaxed);
        if let Some(read_thread_handle) = self.read_thread_handle.take() {
            let _ = read_thread_handle.join();
        }
    }
}

impl<M: MessageContainer> Drop for Device<M> {
    fn drop(&mut self) {
        self.stop_reading_messages()
    }
}

pub trait MessageContainer: Default + Debug + Send + Sync {
    type Message: for<'a> TryFrom<&'a [u8], Error = MessageParseError<'a>> + Debug;
    fn cache_message(&self, message: Self::Message);
    fn wait_for_device_info(&self) -> bool;
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
