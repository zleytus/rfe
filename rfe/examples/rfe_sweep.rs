use rfe::RfExplorer;

fn main() {
    if let Some(rfe) = RfExplorer::connect() {
        println!("{:?}", rfe.wait_for_next_sweep());
    } else {
        eprintln!("Failed to connect to an RF Explorer");
    }
}
