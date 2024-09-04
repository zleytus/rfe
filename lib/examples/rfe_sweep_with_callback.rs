use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use rfe::SpectrumAnalyzer;

fn main() {
    let rfe = SpectrumAnalyzer::connect().expect("RF Explorer should be connected");

    // Create a flag that indicates whether or not a sweep has been received
    let received_sweep = Arc::new(AtomicBool::new(false));
    let received_sweep_clone = Arc::clone(&received_sweep);
    // Set the flag to `true` in the callback that's invoked when a sweep is received
    rfe.set_sweep_callback(move |sweep, start_freq, stop_freq| {
        received_sweep_clone.store(true, Ordering::Relaxed);
        println!("{}-{} Hz", start_freq.as_hz(), stop_freq.as_hz());
        println!("{sweep:?}");
    });

    // Wait until the flag is set to `true`
    while !received_sweep.load(Ordering::Relaxed) {}
}
