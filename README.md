# rfe

`rfe` is a library for communicating with RF Explorer spectrum analyzers and signal generators.

## Example

``` rust
use rfe::RfExplorer;
use std::time::Duration;

fn main() {
    let mut rfe = RfExplorer::connect().expect("RF Explorer should be connected");
    rfe.set_start_stop(90_000_000, 110_000_000).unwrap();

    println!("{:#?}", rfe.config());
    println!("{:#?}", rfe.wait_for_next_sweep(Duration::from_secs(2)));
}
```

## Setup

``` toml
# Cargo.toml
[dependencies]
rfe = { git = "https://github.com/zatchl/rfe" }
```

### Windows and macOS

* Download and install the appropriate [Silicon Labs CP210x USB driver](https://www.silabs.com/products/development-tools/software/usb-to-uart-bridge-vcp-drivers)

### Linux

* The Silicon Labs CP210x driver is included in the kernel
* Make sure the current user has read/write access to the RF Explorer
  * `ls -l /dev/ttyUSB*` to find the RF Explorer's group owner
  * `groups` to see if the current user is a member of the group
  * `gpasswd -a <user> <group>` to add the user to the group
