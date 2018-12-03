use std::fmt::{Display, Formatter, Result as FmtResult};
use std::time::Duration;

pub struct G13Device<'a> {
    device: libusb::Device<'a>,
    handle: libusb::DeviceHandle<'a>
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

        let device = G13Device {
            device: device,
            handle: handle
        };

        device.set_mode_leds(0);
        device.set_led_color(0, 255, 255);
        info!("Initialized {}", device);

        Ok(device)
    }

    pub fn set_mode_leds(&self, leds: i32) {
        let usb_data = [5, leds as u8, 0, 0, 0];

        self.handle.write_control(libusb::request_type(libusb::Direction::Out, libusb::RequestType::Class, libusb::Recipient::Interface),
                9,
                0x305,
                0,
                &usb_data,
                Duration::from_secs(1)
        ).unwrap_or(0);
    }

    pub fn set_led_color(&self, red: u8, green: u8, blue: u8) {
        let usb_data = [5, red, green, blue, 0];

        self.handle.write_control(libusb::request_type(libusb::Direction::Out, libusb::RequestType::Class, libusb::Recipient::Interface),
                9,
                0x307,
                0,
                &usb_data,
                Duration::from_secs(1)
        ).unwrap_or(0);
    }

    pub fn read_keys(&self) -> Result<(), libusb::Error> {
        let mut usb_buffer = [0 as u8; 8];

        match self.handle.read_interrupt(libusb::request_type(libusb::Direction::In, libusb::RequestType::Standard, libusb::Recipient::Device) | 1,
            &mut usb_buffer,
            Duration::from_millis(100)
        ) {
            Ok(_) => Ok(()),
            Err(err) => match err {
                // ignore timeout errors
                libusb::Error::Timeout => Ok(()),
                _ => Err(err)
            }
        }
    }
}

impl<'a> Display for G13Device<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "G13Device(Bus: {:03}, Address: {:03})", self.device.bus_number(), self.device.address())
    }
}

