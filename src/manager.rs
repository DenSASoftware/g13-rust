use crate::device::G13Device;
use crate::key::G13Error;
use crate::constants::*;

use std::thread;
use std::time::Duration;

use log::{error, warn};

pub struct G13Manager {
    context: libusb::Context,
}

impl G13Manager {
    pub fn new() -> Result<G13Manager, libusb::Error> {
        Ok(G13Manager { context: libusb::Context::new()? })
    }

    pub fn find_g13s<'a>(&'a self) -> Vec<G13Device<'a>> {
        let mut list = Vec::new();

        for device in self.context.devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            
            if device_desc.vendor_id() == G13_VENDOR_ID && device_desc.product_id() == G13_PRODUCT_ID {
                list.push(G13Device::from_device(device).unwrap());
            }
        }

        list
    }

    pub fn mainloop(&self) {
        let mut devices = self.find_g13s();

        if devices.is_empty() {
            warn!("Started mainloop without devices, as of now you should connect a G13 and restart this program");
        }

        loop {
            for device in devices.iter_mut() {
                match device.read_keys() {
                    Ok(iter) => {
                        for i in iter {
                            println!("{}", i);
                        }
                    },
                    Err(error) => {
                        match error {
                            G13Error::USBError(libusb::Error::Timeout) => {},
                            _ => {
                                error!("An error occurred: {:?}", error);
                            }
                        }
                    }
                }
            }

            // avoid wasting cpu-cycles when no devices are present
            // maybe add some code to scan for new devices here, because
            // as of now we could simply
            // terminate when no devices are present
            if devices.is_empty() {
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

