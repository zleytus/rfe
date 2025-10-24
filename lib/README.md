# rfe

[![Build and Test](https://github.com/zleytus/rfe/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/zleytus/rfe/actions/workflows/build_and_test.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)

`rfe` is a library for communicating with [RF Explorer](https://www.j3.rf-explorer.com/) spectrum analyzers and signal generators.

## Usage

Add the following to your `Cargo.toml`:

``` toml
[dependencies]
rfe = { git = "https://github.com/zleytus/rfe.git" }
```

### Connecting to an RF Explorer

`rfe` can search for and connect to an attached RF Explorer device, without knowing its port name or baud rate ahead of time, by attempting to connect to all USB serial ports with a VID and PID matching RF Explorer's Silicon Labs CP210x USB to UART Bridge.

```rust
use rfe::{SignalGenerator, SpectrumAnalyzer};

let spectrum_analyzer = SpectrumAnalyzer::connect()?;
let signal_generator = SignalGenerator::connect()?;
```

`rfe` can also connect to an attached RF Explorer device with a known port name and baud rate.

```rust
use rfe::{SignalGenerator, SpectrumAnalyzer};

let spectrum_analyzer = SpectrumAnalyzer::connect_with_name_and_baud_rate("COM2", 500_000)?;
let signal_generator = SignalGenerator::connect_with_name_and_baud_rate("COM1", 500_000)?;
```

### Getting a sweep from an RF Explorer Spectrum Analyzer

`rfe` provides three different APIs for getting a sweep from an RF Explorer spectrum analyzer.

#### `SpectrumAnalyzer::wait_for_next_sweep()`

Synchronously wait for the RF Explorer to measure the next sweep

```rust
use rfe::SpectrumAnalyzer;

let rfe = SpectrumAnalyzer::connect()?;
let sweep = rfe.wait_for_next_sweep();
println!("{:?}", sweep);
```

#### `SpectrumAnalyzer::sweep()`

Return the most recently measured sweep (could be `None` if a sweep hasn't been measured yet)

```rust
use rfe::SpectrumAnalyzer;

let rfe = SpectrumAnalyzer::connect()?;
let sweep = rfe.sweep();
```

#### `SpectrumAnalyzer::set_sweep_callback()`

Set a callback function that gets called, from a separate thread, whenever the RF Explorer measures a sweep

```rust
use rfe::SpectrumAnalyzer;

let rfe = SpectrumAnalyzer::connect()?;
rfe.set_sweep_callback(|sweep, start_freq, stop_freq| {
    println!("Received sweep from {}-{} MHz", start_freq.as_mhz(), stop_freq.as_mhz());
    println!("{:?}", amps);
});
```

### Generating a signal with an RF Explorer Signal Generator

```rust
use rfe::{Frequency, signal_generator::{Attenuation, PowerLevel, SignalGenerator}};

let rfe = SignalGenerator::connect()?;
let result = rfe.start_cw(Frequency::from_mhz(2412), Attenuation::Off, PowerLevel::Low);
```

## Troubleshooting

`rfe` uses the [`tracing`](https://github.com/tokio-rs/tracing) crate to emit structured, event-based diagnostic information that can be collected by executables using the `rfe` library.

## License

This project is dual-licensed under the [MIT License](../LICENSE-MIT) or [Apache 2.0 License](../LICENSE-APACHE).
