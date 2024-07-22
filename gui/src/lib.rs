#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod panels;
mod settings;

mod sweeps;
mod widgets;

pub use app::App;
pub use sweeps::Sweeps;
