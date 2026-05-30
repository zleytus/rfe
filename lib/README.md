# rfe

[![Build and Test](https://github.com/zleytus/rfe/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/zleytus/rfe/actions/workflows/build_and_test.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE-MIT)

`rfe` is a Rust library for communicating with [RF Explorer](https://www.j3.rf-explorer.com/) spectrum analyzers and signal generators over a USB virtual serial port.

It provides high-level device types for RF Explorer hardware and lower-level building blocks for similar serial devices that use the same message container pattern.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rfe = "0.1.0"
```

### Connecting

`rfe` can search for an attached RF Explorer without knowing its port name or baud rate. It tries USB serial ports with the VID and PID used by the RF Explorer's Silicon Labs CP210x USB-to-UART bridge.

```rust
use rfe::{SignalGenerator, SpectrumAnalyzer};

let spectrum_analyzer = SpectrumAnalyzer::connect()?;
let signal_generator = SignalGenerator::connect()?;
```

You can also connect to a known serial port and baud rate.

```rust
use rfe::{SignalGenerator, SpectrumAnalyzer};

let spectrum_analyzer = SpectrumAnalyzer::connect_with_name_and_baud_rate("COM2", 500_000)?;
let signal_generator = SignalGenerator::connect_with_name_and_baud_rate("COM1", 500_000)?;
```

### Spectrum analyzer sweeps

`rfe` provides three APIs for reading spectrum analyzer sweeps.

#### Wait for the next sweep

`SpectrumAnalyzer::wait_for_next_sweep()` blocks until the device reports a new sweep or the timeout elapses.

```rust
use rfe::SpectrumAnalyzer;

let rfe = SpectrumAnalyzer::connect()?;
let sweep = rfe.wait_for_next_sweep()?;
println!("{:?}", sweep);
```

#### Read the latest cached sweep

`SpectrumAnalyzer::sweep()` returns the most recently measured sweep, or `None` if no sweep has been received yet.

```rust
use rfe::SpectrumAnalyzer;

let rfe = SpectrumAnalyzer::connect()?;
let sweep = rfe.sweep();
```

#### Receive sweeps with a callback

`SpectrumAnalyzer::set_sweep_callback()` registers a callback that runs on a separate thread whenever a sweep is received.

```rust
use rfe::SpectrumAnalyzer;

let rfe = SpectrumAnalyzer::connect()?;
rfe.set_sweep_callback(|sweep, start_freq, stop_freq| {
    println!("Received sweep from {}-{} MHz", start_freq.as_mhz(), stop_freq.as_mhz());
    println!("{sweep:?}");
});
```

### Generating a signal with an RF Explorer Signal Generator

```rust
use rfe::{
    signal_generator::{Attenuation, PowerLevel, SignalGenerator},
    Frequency,
};

let rfe = SignalGenerator::connect()?;
rfe.start_cw(Frequency::from_mhz(2412), Attenuation::Off, PowerLevel::Low)?;
```

## Examples

Run the included examples with:

```bash
cargo run -p rfe --example rfe_info
cargo run -p rfe --example rfe_sweep
cargo run -p rfe --example rfe_sweep_with_callback
```

## Troubleshooting

`rfe` uses the [`tracing`](https://github.com/tokio-rs/tracing) crate to emit structured, event-based diagnostic information that can be collected by executables using the `rfe` library.

On Linux, the current user usually needs serial port access through the `dialout` or `uucp` group. See the top-level README for platform setup details.

## License

This project is dual-licensed under the [MIT License](LICENSE-MIT) or [Apache 2.0 License](LICENSE-APACHE).
