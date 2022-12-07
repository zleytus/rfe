use rfe::RfExplorer;

fn main() {
    let rfe = RfExplorer::connect().unwrap();
    loop {
        println!("{:?}\n", rfe.wait_for_next_sweep());
    }
}
