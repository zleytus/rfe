fn main() {
    for spectrum_analyzer in rfe::spectrum_analyzer::RfExplorer::connect_all() {
        println!("{spectrum_analyzer:#?}");
    }

    for signal_generator in rfe::signal_generator::RfExplorer::connect_all() {
        println!("{signal_generator:#?}");
    }
}
