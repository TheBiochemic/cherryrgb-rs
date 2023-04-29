#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use log::error;
use uhid_virt::{Bus, CreateParams, UHIDDevice};

/// Virtual HID device for injecting key events into the Linux HID subsystem
pub struct VirtKbd {
    device: UHIDDevice<std::fs::File>,
}

impl Default for VirtKbd {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtKbd {
    pub fn new() -> Self {
        VirtKbd {
            device: UHIDDevice::create(CreateParams {
                name: String::from("cherryrgb"),
                phys: String::from(""), // ?
                uniq: String::from(""), // ?
                bus: Bus::USB,
                vendor: 0xdead,
                product: 0xbeef,
                version: 0,
                country: 0,
                #[rustfmt::skip]
                rd_data: [
                    // Annotated report descriptor generated by running hid-decode
                    // on the event device of my Cherry MX 10.0 N RGB
                    // hid-decode is part of hid-tools
                    // See: https://gitlab.freedesktop.org/libevdev/hid-tools
                    //
					0x05, 0x01,                    // Usage Page (Generic Desktop)
					0x09, 0x06,                    // Usage (Keyboard)
					0xa1, 0x01,                    // Collection (Application)
					0x85, 0x01,                    //  Report ID (1)
					0x05, 0x07,                    //  Usage Page (Keyboard)
					0x19, 0x04,                    //  Usage Minimum (4)
					0x29, 0x70,                    //  Usage Maximum (112)
					0x15, 0x00,                    //  Logical Minimum (0)
					0x25, 0x01,                    //  Logical Maximum (1)
					0x75, 0x01,                    //  Report Size (1)
					0x95, 0x78,                    //  Report Count (120)
					0x81, 0x02,                    //  Input (Data,Var,Abs)
					0xc0,                          // End Collection
					0x05, 0x01,                    // Usage Page (Generic Desktop)
					0x09, 0x80,                    // Usage (System Control)
					0xa1, 0x01,                    // Collection (Application)
					0x85, 0x02,                    //  Report ID (2)
					0x05, 0x01,                    //  Usage Page (Generic Desktop)
					0x19, 0x81,                    //  Usage Minimum (129)
					0x29, 0x83,                    //  Usage Maximum (131)
					0x15, 0x00,                    //  Logical Minimum (0)
					0x25, 0x01,                    //  Logical Maximum (1)
					0x95, 0x03,                    //  Report Count (3)
					0x75, 0x01,                    //  Report Size (1)
					0x81, 0x02,                    //  Input (Data,Var,Abs)
					0x95, 0x01,                    //  Report Count (1)
					0x75, 0x05,                    //  Report Size (5)
					0x81, 0x01,                    //  Input (Cnst,Arr,Abs)
					0xc0,                          // End Collection
					0x05, 0x0c,                    // Usage Page (Consumer Devices)
					0x09, 0x01,                    // Usage (Consumer Control)
					0xa1, 0x01,                    // Collection (Application)
					0x85, 0x03,                    //  Report ID (3)
					0x15, 0x00,                    //  Logical Minimum (0)
					0x26, 0xff, 0x1f,              //  Logical Maximum (8191)
					0x19, 0x00,                    //  Usage Minimum (0)
					0x2a, 0xff, 0x1f,              //  Usage Maximum (8191)
					0x75, 0x10,                    //  Report Size (16)
					0x95, 0x01,                    //  Report Count (1)
					0x81, 0x00,                    //  Input (Data,Arr,Abs)
					0xc0,                          // End Collection
					0x06, 0x1c, 0xff,              // Usage Page (Vendor Usage Page 0xff1c)
					0x09, 0x92,                    // Usage (Vendor Usage 0x92)
					0xa1, 0x01,                    // Collection (Application)
					0x85, 0x04,                    //  Report ID (4)
					0x19, 0x00,                    //  Usage Minimum (0)
					0x2a, 0xff, 0x00,              //  Usage Maximum (255)
					0x15, 0x00,                    //  Logical Minimum (0)
					0x26, 0xff, 0x00,              //  Logical Maximum (255)
					0x75, 0x08,                    //  Report Size (8)
					0x95, 0x3f,                    //  Report Count (63)
					0x91, 0x00,                    //  Output (Data,Arr,Abs)
					0x19, 0x00,                    //  Usage Minimum (0)
					0x29, 0xff,                    //  Usage Maximum (255)
					0x81, 0x00,                    //  Input (Data,Arr,Abs)
					0xc0,                          // End Collection
					0x05, 0x01,                    // Usage Page (Generic Desktop)
					0x09, 0x02,                    // Usage (Mouse)
					0xa1, 0x01,                    // Collection (Application)
					0x85, 0x05,                    //  Report ID (5)
					0x09, 0x01,                    //  Usage (Pointer)
					0xa1, 0x00,                    //  Collection (Physical)
					0x05, 0x09,                    //   Usage Page (Button)
					0x19, 0x01,                    //   Usage Minimum (1)
					0x29, 0x05,                    //   Usage Maximum (5)
					0x15, 0x00,                    //   Logical Minimum (0)
					0x25, 0x01,                    //   Logical Maximum (1)
					0x95, 0x05,                    //   Report Count (5)
					0x75, 0x01,                    //   Report Size (1)
					0x81, 0x02,                    //   Input (Data,Var,Abs)
					0x95, 0x01,                    //   Report Count (1)
					0x75, 0x03,                    //   Report Size (3)
					0x81, 0x01,                    //   Input (Cnst,Arr,Abs)
					0x05, 0x01,                    //   Usage Page (Generic Desktop)
					0x09, 0x30,                    //   Usage (X)
					0x09, 0x31,                    //   Usage (Y)
					0x16, 0x00, 0x80,              //   Logical Minimum (-32768)
					0x26, 0xff, 0x7f,              //   Logical Maximum (32767)
					0x75, 0x10,                    //   Report Size (16)
					0x95, 0x02,                    //   Report Count (2)
					0x81, 0x06,                    //   Input (Data,Var,Rel)
					0x09, 0x38,                    //   Usage (Wheel)
					0x15, 0x81,                    //   Logical Minimum (-127)
					0x25, 0x7f,                    //   Logical Maximum (127)
					0x75, 0x08,                    //   Report Size (8)
					0x95, 0x01,                    //   Report Count (1)
					0x81, 0x06,                    //   Input (Data,Var,Rel)
					0x05, 0x0c,                    //   Usage Page (Consumer Devices)
					0x0a, 0x38, 0x02,              //   Usage (AC Pan)
					0x15, 0x81,                    //   Logical Minimum (-127)
					0x25, 0x7f,                    //   Logical Maximum (127)
					0x75, 0x08,                    //   Report Size (8)
					0x95, 0x01,                    //   Report Count (1)
					0x81, 0x06,                    //   Input (Data,Var,Rel)
					0xc0,                          //  End Collection
					0xc0,                          // End Collection
                ]
                .to_vec(),
            })
            .map_err(|err| error!("Could not create VirtKbd: {:?}", err))
            .expect("Could not create VirtKbd"),
        }
    }

    /// Forward a single HID event.
    /// See: CherryKeyboard::forward_filtered_keys()
    pub fn forward(&mut self, input: &[u8]) {
        self.device.write(input).unwrap();
    }
}
