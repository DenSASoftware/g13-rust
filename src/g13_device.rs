use crate::constants::*;
use crate::g13_key::{G13Key, G13_KEYS};

use log::{info, error};

use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::time::Duration;

pub struct G13Device<'a> {
    device: libusb::Device<'a>,
    handle: libusb::DeviceHandle<'a>,
    keys: [G13Key; G13_KEYS_LENGTH]
}

impl<'a> G13Device<'a> {
    pub fn from_device(device: libusb::Device<'a>) -> Result<G13Device<'a>, libusb::Error> {
        let mut handle = match device.open() {
            Ok(handle) => handle,
            Err(error) => {
                error!("Can't open device on bus {:03} address {:03}", device.bus_number(), device.address());
                return Err(error)
            }
        };

        let interface = 0;
        if handle.kernel_driver_active(interface)? {
            handle.detach_kernel_driver(interface)?;
        }
        handle.claim_interface(interface)?;

        // since rust still doesn't allow to initialize an sized array without default value of a
        // cloneable type and the clone-trait on the key caused problems in the past we have this
        // monstrosity
        let keys: [G13Key; G13_KEYS_LENGTH] = [
            G13Key::new(), G13Key::new(), G13Key::new(), G13Key::new(),
            G13Key::new(), G13Key::new(), G13Key::new(), G13Key::new(),
            G13Key::new(), G13Key::new(), G13Key::new(), G13Key::new(),
            G13Key::new(), G13Key::new(), G13Key::new(), G13Key::new(),
            G13Key::new(), G13Key::new(), G13Key::new(), G13Key::new(),
            G13Key::new(), G13Key::new(), G13Key::new(), G13Key::new(),
            G13Key::new(), G13Key::new(), G13Key::new(), G13Key::new(),
            G13Key::new(), G13Key::new(), G13Key::new(), G13Key::new(),
            G13Key::new(), G13Key::new(), G13Key::new(), G13Key::new(),
            G13Key::new(), G13Key::new(), G13Key::new(), G13Key::new(),
        ];
        let device = G13Device {
            device: device,
            handle: handle,
            keys: keys
        };

        device.set_mode_leds(0);
        device.set_led_color(0, 255, 255);
        info!("Initialized {:?}", device);

        Ok(device)
    }

    pub fn set_mode_leds(&self, leds: i32) {
        let usb_data = [5, leds as u8, 0, 0, 0];

        self.handle.write_control(G13_LED_MODE_ENDPOINT, 9, 0x305, 0, &usb_data, Duration::from_secs(1)).unwrap_or(0);
    }

    pub fn set_led_color(&self, red: u8, green: u8, blue: u8) {
        let usb_data = [5, red, green, blue, 0];

        self.handle.write_control(G13_LED_ENDPOINT, 9, 0x307, 0, &usb_data, Duration::from_secs(1)).unwrap_or(0);
    }

    pub fn read_keys(&mut self) -> Result<(), libusb::Error> {
        let mut usb_buffer = [0 as u8; 8];

        match self.handle.read_interrupt(G13_KEYS_ENDPOINT, &mut usb_buffer, Duration::from_millis(100)) {
            Ok(_) => {
                self.process_keys(&usb_buffer);
                Ok(())
            },
            Err(err) => match err {
                // ignore timeout errors
                libusb::Error::Timeout => Ok(()),
                _ => Err(err)
            }
        }
    }

    fn process_keys(&mut self, bytes: &[u8; 8]) {
        for i in 0..G13_KEYS_LENGTH {
            // ignore some inputs that aren't really keys
            if i == 22 || i == 23 || i > 35 {
                continue
            }

            let byte = bytes[3 + (i / 8)];
            let bit = byte & (1 << (i % 8));
            let pressed = bit != 0;

            let mut key = &mut self.keys[i];
            if pressed != key.is_pressed {
                let key_info = &G13_KEYS[i];
                match key.is_pressed {
                    true => key.released(key_info.name),
                    false => key.pressed(key_info.name)
                }

                key.is_pressed = !key.is_pressed;
            }
        }
    }
}

impl<'a> Debug for G13Device<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "G13Device(Bus: {:03}, Address: {:03})", self.device.bus_number(), self.device.address())
    }
}

