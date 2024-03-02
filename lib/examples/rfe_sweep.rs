use rfe::SpectrumAnalyzer;

fn main() {
    let rfe = SpectrumAnalyzer::connect().expect("RF Explorer should be connected");
    println!("{:?}", rfe.wait_for_next_sweep());
}
