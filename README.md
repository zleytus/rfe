# rfe

`rfe` is a set of tools for communicating with [RF Explorer](https://www.j3.rf-explorer.com/) spectrum analyzers and signal generators.

* [lib/](lib/): A Rust library for communicating with RF Explorer devices
* [ffi/](ffi/): A C-compatible wrapper library around the Rust library for use with other languages
* [gui/](gui/): A GUI for visualizing signals measured by RF Explorer spectrum analyzers

## Build

The tools in `rfe` are written in Rust and part of the same [Cargo](https://github.com/rust-lang/cargo) workspace. Running `cargo build` or `cargo build --release` in this repository's top-level directory builds each tool and puts them in the same output directory (`target/debug/` or `target/release`).

## Requirements

In order to communicate with an RF Explorer device over USB using its [serial protocol](https://github.com/RFExplorer/RFExplorer-for-.NET/wiki/RF-Explorer-UART-API-interface-specification), a driver for the RF Explorer's [Silicon Labs CP210x USB to UART Bridge](https://www.silabs.com/developer-tools/usb-to-uart-bridge-vcp-drivers) must be installed.

### Windows

Download and install the appropriate [Silicon Labs CP210x USB to UART Bridge driver](https://www.silabs.com/developer-tools/usb-to-uart-bridge-vcp-drivers).

### macOS

As of macOS 10.15, Apple provides its own built-in driver for Silicon Labs CP210x devices. If the built-in driver doesn't work, download and install the appropriate [Silicon Labs CP210x USB to UART Bridge driver](https://www.silabs.com/developer-tools/usb-to-uart-bridge-vcp-drivers).

### Linux

The Linux kernel includes a driver for Silicon Labs CP210x devices, but `rfe` requires the following additional steps in order to find and communicate with an attached RF Explorer.

#### Install `pkg-config` and `udev` header files to provide serial port enumeration and USB device information

| Distro             | Command                                           |
| ------------------ | ------------------------------------------------- |
| Debian/Ubuntu/Mint | `apt install pkg-config libudev-dev`              |
| Fedora/CentOS/RHEL | `dnf install pkgconf-pkg-config systemd-devel`    |
| openSUSE           | `zypper install pkgconf-pkg-config systemd-devel` |
| Arch/Manjaro       | `pacman -Syu pkgconf systemd`                     |

#### Add the current user to the `dialout` or `uucp` group to get permission to access serial ports

| Distro             | Command                         |
| ------------------ | ------------------------------- |
| Debian/Ubuntu/Mint | `gpasswd -a <username> dialout` |
| Fedora/CentOS/RHEL | `gpasswd -a <username> dialout` |
| openSUSE           | `gpasswd -a <username> dialout` |
| Arch/Manjaro       | `gpasswd -a <username> uucp`    |

## License

This project is dual-licensed under the [MIT License](LICENSE-MIT) or [Apache 2.0 License](LICENSE-APACHE).
