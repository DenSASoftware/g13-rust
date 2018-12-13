extern crate libusb;
extern crate log;
extern crate simple_logger;

mod g13_manager;
mod g13_device;
mod g13_key;
mod constants;

use crate::g13_manager::G13Manager;

fn main() {
    // enable logging
    simple_logger::init().unwrap();

    let manager = G13Manager::new().unwrap();
    manager.mainloop();
}
