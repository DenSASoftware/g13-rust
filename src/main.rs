extern crate libusb;
#[macro_use]
extern crate log;
extern crate simple_logger;

mod g13_manager;
mod g13_device;

use g13_manager::G13Manager;
use std::thread;
use std::time::Duration;

fn main() {
    simple_logger::init().unwrap();

    let manager = G13Manager::new().unwrap();
    
    let mut g13s = manager.find_g13s();
    println!("Found {} G13-Devices", g13s.len());

    if g13s.len() > 0 {
        let mine = g13s.pop().unwrap();

        loop {
            match mine.read_keys() {
                Ok(_) => info!("Read keys correctly"),
                Err(error) => error!("An error occurred: {}", error)
            }

            thread::sleep(Duration::from_millis(100));
        }
    }
}
