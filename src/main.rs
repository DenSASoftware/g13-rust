extern crate libusb;
#[macro_use]
extern crate log;
extern crate simple_logger;

mod g13_manager;
mod g13_device;

use g13_manager::G13Manager;

fn main() {
    simple_logger::init().unwrap();

    let manager = G13Manager::new().unwrap();
    
    let g13s = manager.find_g13s();
    println!("Found {} G13-Devices", g13s.len());
}
