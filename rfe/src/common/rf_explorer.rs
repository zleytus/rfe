macro_rules! rf_explorer_impl {
    ($device:ty) => {
        pub(crate) const NEXT_SCREEN_DATA_TIMEOUT: Duration = Duration::from_secs(2);

        /// Connects to the first available RF Explorer.
        #[tracing::instrument(ret)]
        pub fn connect() -> Option<Self> {
            use $crate::common::{serial_port, SerialPort};
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
                    let device = <$device>::connect(serial_port).ok()?;
                    Some(Self { device })
                })
        }

        /// Connects to an RF Explorer with the provided name and baud rate.
        #[tracing::instrument(ret, err)]
        pub fn connect_with_name_and_baud_rate(
            name: &str,
            baud_rate: u32,
        ) -> $crate::common::ConnectionResult<Self> {
            let serial_port = $crate::common::SerialPort::open_with_name(name, baud_rate)?;
            let device = <$device>::connect(serial_port)?;
            Ok(Self { device })
        }

        /// Connects to all available RF Explorers.
        #[tracing::instrument(ret)]
        pub fn connect_all() -> Vec<Self> {
            use $crate::common::{serial_port, SerialPort};
            serial_port::silabs_cp210x_ports()
                .flat_map(|port_info| {
                    [
                        (port_info.clone(), serial_port::SLOW_BAUD_RATE),
                        (port_info, serial_port::FAST_BAUD_RATE),
                    ]
                })
                .filter_map(|(port_info, baud_rate)| {
                    let serial_port = SerialPort::open(&port_info, baud_rate).ok()?;
                    let device = <$device>::connect(serial_port).ok()?;
                    Some(Self { device })
                })
                .collect()
        }

        /// Sends bytes to the RF Explorer.
        #[tracing::instrument(skip(self, bytes))]
        pub fn send_bytes(&self, bytes: impl AsRef<[u8]> + std::fmt::Debug) -> io::Result<()> {
            self.device.serial_port().send_bytes(bytes)
        }

        /// The name of the serial port used by the RF Explorer.
        #[tracing::instrument(skip(self))]
        pub fn port_name(&self) -> &str {
            &self.device.serial_port().port_info().port_name
        }

        /// Returns the RF Explorer's firmware version.
        #[tracing::instrument(skip(self))]
        pub fn firmware_version(&self) -> String {
            if let Some(setup_info) = self.device.setup_info.0.lock().unwrap().as_ref() {
                setup_info.firmware_version.clone()
            } else {
                String::default()
            }
        }

        /// Returns the RF Explorer's serial number.
        #[tracing::instrument(skip(self))]
        pub fn serial_number(&self) -> io::Result<$crate::common::SerialNumber> {
            if let Some(ref serial_number) = *self.device.serial_number.0.lock().unwrap() {
                return Ok(serial_number.clone());
            }

            self.device
                .serial_port()
                .send_command(crate::common::Command::RequestSerialNumber)?;

            let (lock, cvar) = &self.device.serial_number;
            tracing::trace!("Waiting to receive SerialNumber from RF Explorer");
            let _ = cvar
                .wait_timeout_while(
                    lock.lock().unwrap(),
                    std::time::Duration::from_secs(2),
                    |serial_number| serial_number.is_none(),
                )
                .unwrap();

            if let Some(ref serial_number) = *self.device.serial_number.0.lock().unwrap() {
                Ok(serial_number.clone())
            } else {
                Err(io::ErrorKind::TimedOut.into())
            }
        }

        /// Turns on the RF Explorer's LCD screen.
        #[tracing::instrument(skip(self))]
        pub fn lcd_on(&self) -> io::Result<()> {
            self.device
                .serial_port()
                .send_command($crate::common::Command::EnableLcd)
        }

        /// Turns off the RF Explorer's LCD screen.
        #[tracing::instrument(skip(self))]
        pub fn lcd_off(&self) -> io::Result<()> {
            self.device
                .serial_port()
                .send_command($crate::common::Command::DisableLcd)
        }

        /// Requests the RF Explorer start sending screen data.
        #[tracing::instrument(skip(self))]
        pub fn enable_dump_screen(&self) -> io::Result<()> {
            self.device
                .serial_port()
                .send_command($crate::common::Command::EnableDumpScreen)
        }

        /// Requests the RF Explorer stop sending screen data.
        #[tracing::instrument(skip(self))]
        pub fn disable_dump_screen(&self) -> io::Result<()> {
            self.device
                .serial_port()
                .send_command($crate::common::Command::DisableDumpScreen)
        }

        /// Stops the RF Explorer data dump.
        #[tracing::instrument(skip(self))]
        pub fn hold(&self) -> io::Result<()> {
            self.device
                .serial_port()
                .send_command($crate::common::Command::Hold)
        }

        /// Returns the baud rate of the serial connection to the RF Explorer.
        pub fn baud_rate(&self) -> io::Result<u32> {
            self.device.serial_port().baud_rate()
        }

        /// Sets the baud rate of the serial connection to the RF Explorer.
        ///
        /// Valid baud rates are 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, and 500000 bps.
        pub fn set_baud_rate(&self, baud_rate: u32) -> $crate::Result<()> {
            let baud_rate = $crate::common::BaudRate::try_from(baud_rate)?;
            self.device
                .serial_port()
                .send_command($crate::common::Command::SetBaudRate { baud_rate })?;
            self.device
                .serial_port()
                .set_baud_rate(baud_rate.bps())
                .map_err($crate::Error::from)
        }

        /// Reboots the RF Explorer.
        #[tracing::instrument(skip(self))]
        pub fn reboot(self) -> io::Result<()> {
            self.device
                .serial_port()
                .send_command($crate::common::Command::Reboot)
        }

        /// Turns off the RF Explorer.
        #[tracing::instrument(skip(self))]
        pub fn power_off(self) -> io::Result<()> {
            self.device
                .serial_port()
                .send_command($crate::common::Command::PowerOff)
        }
    };
}

pub(crate) use rf_explorer_impl;
