use rfe::{RfExplorer, SpectrumAnalyzer};
use std::time::Duration;

fn main() {
    let rfe = RfExplorer::<SpectrumAnalyzer>::connect().expect("RF Explorer should be connected");

    loop {
        let sweep = rfe.wait_for_next_sweep(Duration::from_secs(2)).unwrap();
        println!("{:?}\n", sweep.amplitudes_dbm());
    }
}
