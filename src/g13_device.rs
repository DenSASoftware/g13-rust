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

        Ok(G13Device {
            device: device,
            handle: handle
        })
    }
}

