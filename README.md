# rfe

`rfe` is a library for communicating with [RF Explorer](https://www.j3.rf-explorer.com/) spectrum analyzers and signal generators.

## Usage

``` rust
use rfe::RfExplorer;
use std::time::Duration;

fn main() {
    let mut rfe = RfExplorer::connect().expect("RF Explorer should be connected");
    rfe.set_start_stop(90_000_000, 110_000_000).unwrap();

    println!("{:#?}", rfe.config());
    println!("{:#?}", rfe.wait_for_next_sweep(Duration::from_secs(2)));
}
```

## Requirements

### Windows and macOS

Download and install the appropriate [Silicon Labs CP210x USB driver](https://www.silabs.com/products/development-tools/software/usb-to-uart-bridge-vcp-drivers)

### Linux

#### Install `pkg-config` and `udev` header files

| Distro             | Command                                           |
|--------------------|---------------------------------------------------|
| Debian/Ubuntu      | `apt install pkg-config libudev-dev`              |
| Fedora/CentOS/RHEL | `dnf install pkgconf-pkg-config systemd-devel`    |
| openSUSE           | `zypper install pkgconf-pkg-config systemd-devel` |
| Arch/Manjaro       | `pacman -Syu pkgconf systemd`                     |

#### Add yourself to the `dialout` or `uucp` group to get permission to access the RF Explorer

| Distro             | Command                         |
|--------------------|---------------------------------|
| Debian/Ubuntu      | `gpasswd -a <username> dialout` |
| Fedora/CentOS/RHEL | `gpasswd -a <username> dialout` |
| openSUSE           | `gpasswd -a <username> dialout` |
| Arch/Manjaro       | `gpasswd -a <username> uucp`    |
