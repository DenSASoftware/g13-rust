extern crate libusb;
extern crate uinput;

extern crate log;
extern crate simple_logger;

mod manager;
mod device;
mod key;
mod constants;

use crate::manager::G13Manager;

fn main() {
    // enable logging
    simple_logger::init().unwrap();

    let manager = G13Manager::new().unwrap();
    manager.mainloop();
}
