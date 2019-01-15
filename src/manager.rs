use crate::device::G13Device;
use crate::key::{G13Error, G13Button, G13KeyAction, G13KeyPress};
use crate::constants::*;
use crate::profile::G13KeyProfile;

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

        let mut uinput_device = uinput::default().unwrap().name("G13").unwrap().event(uinput::event::Keyboard::All).unwrap().create().unwrap();
        let (click_sender, click_recv) = std::sync::mpsc::channel();
        let mut profile = G13KeyProfile::new();

        std::thread::spawn(move || {
            for (button, press_type) in click_recv.iter() {
                match button {
                    G13Button::L1 | G13Button::L2 | G13Button::L3 => {
                        if press_type == G13KeyPress::Released {
                            let page = usize::from(button) - usize::from(G13Button::L1);

                            profile.set_page(page);
                        }
                    },
                    _ => {
                        match profile.get_action(button) {
                            G13KeyAction::Key(key) => {
                                match press_type {
                                    G13KeyPress::Pressed => uinput_device.press(key).unwrap(),
                                    G13KeyPress::Released => uinput_device.release(key).unwrap(),
                                }
                                uinput_device.synchronize().unwrap();
                            },
                            _ => {},
                        }
                    }
                }

                uinput_device.synchronize().unwrap();
            }
        });

        loop {
            for device in devices.iter_mut() {
                match device.read_keys() {
                    Ok(iter) => {
                        for i in iter {
                            click_sender.send(i).unwrap();
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

