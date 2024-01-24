use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use rfe::RfExplorer;

fn main() {
    let Some(rfe) = RfExplorer::connect() else {
        eprintln!("Failed to connect to an RF Explorer");
        return;
    };

    let received_sweep = Arc::new(AtomicBool::new(false));
    let received_sweep_clone = Arc::clone(&received_sweep);
    rfe.set_sweep_callback(move |sweep| {
        received_sweep_clone.store(true, Ordering::Relaxed);
        println!("{sweep:?}");
    });

    // Wait to receive a sweep before exiting
    while !received_sweep.load(Ordering::Relaxed) {}
}
