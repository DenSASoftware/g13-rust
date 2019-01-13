use crate::constants::*;
use crate::key::{G13Key, G13Error};

use log::{info, error};

use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::time::Duration;

pub struct G13Device<'a> {
    device: libusb::Device<'a>,
    handle: libusb::DeviceHandle<'a>,
    pub input: uinput::Device,
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

        let input_device = uinput::default().unwrap().name("G13").unwrap().event(uinput::event::Keyboard::All).unwrap().create().unwrap();
        let device = G13Device {
            device: device,
            handle: handle,
            input: input_device,
            keys: keys
        };

        device.init_lcd();
        device.set_mode_leds(0);
        device.set_led_color(0, 255, 255);
        device.clear_lcd();
        info!("Initialized {:?}", device);

        Ok(device)
    }

    fn init_lcd(&self) {
        let _dummy_arr = [0u8; 0];
        self.handle.write_control(0, 9, 1, 0, &_dummy_arr, Duration::from_secs(1)).unwrap_or(0);
    }

    pub fn clear_lcd(&self) {
        let lcd_data = [0; G13_LCD_ARRAY_LEN];
        self.write_lcd(&lcd_data);
    }

    pub fn write_lcd(&self, pixels: &[u8; G13_LCD_ARRAY_LEN]) {
        let mut usb_data = [0u8; G13_LCD_BUFFER_LEN];
        usb_data[0] = 3;
        usb_data[G13_LCD_BUFFER_PADDING..].copy_from_slice(pixels);

        self.handle.write_interrupt(G13_LCD_ENDPOINT, &mut usb_data, Duration::from_secs(1)).unwrap_or(0);
    }

    pub fn write_lcd2d(&self, pixels: &[[u8; G13_LCD_WIDTH]; G13_LCD_HEIGHT_IN_BYTES]) {
        let mut usb_data = [0u8; G13_LCD_BUFFER_LEN];
        usb_data[0] = 3;
        for i in 0..G13_LCD_HEIGHT_IN_BYTES {
            usb_data[G13_LCD_BUFFER_PADDING + G13_LCD_WIDTH * i..G13_LCD_BUFFER_PADDING + G13_LCD_WIDTH * (i + 1)].copy_from_slice(&pixels[i]);
        }

        self.handle.write_interrupt(G13_LCD_ENDPOINT, &mut usb_data, Duration::from_secs(1)).unwrap_or(0);
    }

    pub fn set_mode_leds(&self, leds: i32) {
        let usb_data = [5, leds as u8, 0, 0, 0];

        self.handle.write_control(G13_LED_MODE_ENDPOINT, 9, 0x305, 0, &usb_data, Duration::from_secs(1)).unwrap_or(0);
    }

    pub fn set_led_color(&self, red: u8, green: u8, blue: u8) {
        let usb_data = [5, red, green, blue, 0];

        self.handle.write_control(G13_LED_ENDPOINT, 9, 0x307, 0, &usb_data, Duration::from_secs(1)).unwrap_or(0);
    }

    pub fn read_keys(&mut self) -> Result<(), G13Error> {
        let mut usb_buffer = [0 as u8; 8];

        match self.handle.read_interrupt(G13_KEYS_ENDPOINT, &mut usb_buffer, Duration::from_millis(100)) {
            Ok(_) => {
                self.process_keys(&usb_buffer);
                Ok(())
            },
            Err(err) => match err {
                // ignore timeout errors
                libusb::Error::Timeout => Ok(()),
                _ => Err(err.into())
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

            let key_pressed = self.keys[i].is_pressed;
            if pressed != key_pressed {
                let key_action = &self.keys[i].action.clone();
                match key_pressed {
                    true => key_action.released(self).unwrap(),
                    false => key_action.pressed(self).unwrap()
                }

                self.keys[i].is_pressed = !key_pressed;
            }
        }
    }
}

impl<'a> Debug for G13Device<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "G13Device(Bus: {:03}, Address: {:03})", self.device.bus_number(), self.device.address())
    }
}

