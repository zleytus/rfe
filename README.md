# rfe

[![Build and Test](https://github.com/zleytus/rfe/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/zleytus/rfe/actions/workflows/build_and_test.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)

`rfe` is a Rust library and GUI for [RF Explorer](https://www.j3.rf-explorer.com/) spectrum analyzers and signal generators.

## Crates

### [`rfe`](lib/)

Rust library for RF Explorer communication

```rust
use rfe::SpectrumAnalyzer;

let rfe = SpectrumAnalyzer::connect()?;
let sweep = rfe.wait_for_next_sweep()?;
```

### [`rfe-ffi`](ffi/)

C-compatible FFI for use with other languages

```c
#include "rfe.h"

SpectrumAnalyzer *rfe = rfe_spectrum_analyzer_connect();
if (rfe) {
    uint16_t sweep_len = rfe_spectrum_analyzer_sweep_len(rfe);
    float *sweep = malloc(sizeof(float) * sweep_len);
    rfe_spectrum_analyzer_wait_for_next_sweep(rfe, sweep, sweep_len, NULL);
    
    // Use sweep

    free(sweep);
    rfe_spectrum_analyzer_free(rfe);
}
```

### [`rfe-gui`](gui/)

GUI for visualizing spectrum analyzer data

![rfe-gui screenshot](./gui/assets/rfe-gui.jpg)

## Build

Build all crates in the workspace with:

```bash
cargo build --release
```

Outputs will be in `target/release/`.

## Requirements

To communicate with RF Explorer devices over USB, you need a driver for its CP210x USB-to-UART bridge.

### Windows

Download and install the [Silicon Labs CP210x driver](https://www.silabs.com/developer-tools/usb-to-uart-bridge-vcp-drivers).

### macOS

macOS 10.15+ includes a built-in driver for CP210x devices. If it doesn't work, install the [Silicon Labs CP210x driver](https://www.silabs.com/developer-tools/usb-to-uart-bridge-vcp-drivers).

### Linux

The kernel includes the CP210x driver, but additional setup is required:

#### 1. Install dependencies

`rfe` uses `pkg-config` and `udev` header files to provide serial port enumeration and USB device information

| Distro             | Command                                           |
| ------------------ | ------------------------------------------------- |
| Debian/Ubuntu/Mint | `apt install pkg-config libudev-dev`              |
| Fedora/CentOS/RHEL | `dnf install pkgconf-pkg-config systemd-devel`    |
| openSUSE           | `zypper install pkgconf-pkg-config systemd-devel` |
| Arch/Manjaro       | `pacman -Syu pkgconf systemd`                     |

#### 2. Grant serial port access

The current user must belong to the `dialout` or `uucp` group to get permission to access serial ports

| Distro             | Command                         |
| ------------------ | ------------------------------- |
| Debian/Ubuntu/Mint | `gpasswd -a <username> dialout` |
| Fedora/CentOS/RHEL | `gpasswd -a <username> dialout` |
| openSUSE           | `gpasswd -a <username> dialout` |
| Arch/Manjaro       | `gpasswd -a <username> uucp`    |

**Note:** Log out and back in (or reboot) for group changes to take effect.

## License

This project is dual-licensed under the [MIT License](LICENSE-MIT) or [Apache 2.0 License](LICENSE-APACHE).
