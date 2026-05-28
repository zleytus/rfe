//! Communicate with RF Explorer spectrum analyzers and signal generators.
//!
//! Use [`SpectrumAnalyzer`] for RF Explorer spectrum analyzer devices and
//! [`SignalGenerator`] for RF Explorer signal generator devices. Frequencies are
//! represented with [`Frequency`].
//!
//! # Examples
//!
//! ```no_run
//! use rfe::{Frequency, SpectrumAnalyzer};
//!
//! let rfe = SpectrumAnalyzer::connect().expect("RF Explorer should be connected");
//! rfe.set_center_span(Frequency::from_mhz(100), Frequency::from_mhz(20))?;
//! let sweep = rfe.wait_for_next_sweep()?;
//! # Ok::<(), rfe::Error>(())
//! ```
//!
//! # Extension API
//!
//! [`Device`] and [`MessageContainer`] provide the lower-level serial device
//! framework used by the high-level RF Explorer types. They can be reused for
//! RF Explorer-like devices that expose compatible serial message streams.

mod common;
mod rf_explorer;

/// RF Explorer signal generator types and commands.
pub mod signal_generator;
/// RF Explorer spectrum analyzer types and commands.
pub mod spectrum_analyzer;

pub use common::*;
pub use rf_explorer::ScreenData;
pub use signal_generator::SignalGenerator;
pub use spectrum_analyzer::SpectrumAnalyzer;
