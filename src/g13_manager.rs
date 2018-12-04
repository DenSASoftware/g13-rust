use g13_device::G13Device;
use constants::*;

pub struct G13Manager {
    context: libusb::Context,
}

impl G13Manager {
    pub fn new() -> Result<G13Manager, libusb::Error> {
        Ok(G13Manager { context: libusb::Context::new()? })
    }

    pub fn find_g13s<'a>(&'a self) -> Vec<G13Device<'a>> {
        let mut list = Vec::new();

        for mut device in self.context.devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            
            if device_desc.vendor_id() == G13_VENDOR_ID && device_desc.product_id() == G13_PRODUCT_ID {
                list.push(G13Device::from_device(device).unwrap());
            }
        }

        list
    }

    pub fn mainloop_with_devices<'a>(&self, devices: &mut Vec<G13Device<'a>>) {
        loop {
            for device in devices.iter_mut() {
               if let Err(error) = device.read_keys() { 
                    error!("An error occurred: {}", error);
                }
            }
        }
    }

    pub fn mainloop(&self) {
        self.mainloop_with_devices(&mut self.find_g13s());
    }
}

