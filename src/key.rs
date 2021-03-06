use crate::device::G13Device;

#[derive(Debug)]
pub enum G13Button {
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    G9,
    G10,
    G11,
    G12,
    G13,
    G14,
    G15,
    G16,
    G17,
    G18,
    G19,
    G20,
    G21,
    G22,
    UNDEF1,
    LIGHTSTATE,
    BD,
    L1,
    L2,
    L3,
    L4,
    M1,
    M2,
    M3,
    MR,
    LEFT,
    DOWN,
    TOP,
    UNDEF3,
    LIGHT,
    LIGHT2,
    MISCTOGGLE,
}

impl From<usize> for G13Button {
    fn from(i: usize) -> G13Button {
        match i {
            00 => G13Button::G1,
            01 => G13Button::G2,
            02 => G13Button::G3,
            03 => G13Button::G4,
            04 => G13Button::G5,
            05 => G13Button::G6,
            06 => G13Button::G7,
            07 => G13Button::G8,
            08 => G13Button::G9,
            09 => G13Button::G10,
            10 => G13Button::G11,
            11 => G13Button::G12,
            12 => G13Button::G13,
            13 => G13Button::G14,
            14 => G13Button::G15,
            15 => G13Button::G16,
            16 => G13Button::G17,
            17 => G13Button::G18,
            18 => G13Button::G19,
            19 => G13Button::G20,
            20 => G13Button::G21,
            21 => G13Button::G22,
            22 => G13Button::UNDEF1,
            23 => G13Button::LIGHTSTATE,
            24 => G13Button::BD,
            25 => G13Button::L1,
            26 => G13Button::L2,
            27 => G13Button::L3,
            28 => G13Button::L4,
            29 => G13Button::M1,
            30 => G13Button::M2,
            31 => G13Button::M3,
            32 => G13Button::MR,
            33 => G13Button::LEFT,
            34 => G13Button::DOWN,
            35 => G13Button::TOP,
            36 => G13Button::UNDEF3,
            37 => G13Button::LIGHT,
            38 => G13Button::LIGHT2,
            39 => G13Button::MISCTOGGLE,
            _ => panic!("the coder was lazy and now you need to pass a value in range 0..40")
        }
    }
}

impl From<G13Button> for usize {
    fn from(b: G13Button) -> usize {
        match b {
            G13Button::G1 => 00,
            G13Button::G2 => 01,
            G13Button::G3 => 02,
            G13Button::G4 => 03,
            G13Button::G5 => 04,
            G13Button::G6 => 05,
            G13Button::G7 => 06,
            G13Button::G8 => 07,
            G13Button::G9 => 08,
            G13Button::G10 => 09,
            G13Button::G11 => 10,
            G13Button::G12 => 11,
            G13Button::G13 => 12,
            G13Button::G14 => 13,
            G13Button::G15 => 14,
            G13Button::G16 => 15,
            G13Button::G17 => 16,
            G13Button::G18 => 17,
            G13Button::G19 => 18,
            G13Button::G20 => 19,
            G13Button::G21 => 20,
            G13Button::G22 => 21,
            G13Button::UNDEF1 => 22,
            G13Button::LIGHTSTATE => 23,
            G13Button::BD => 24,
            G13Button::L1 => 25,
            G13Button::L2 => 26,
            G13Button::L3 => 27,
            G13Button::L4 => 28,
            G13Button::M1 => 29,
            G13Button::M2 => 30,
            G13Button::M3 => 31,
            G13Button::MR => 32,
            G13Button::LEFT => 33,
            G13Button::DOWN => 34,
            G13Button::TOP => 35,
            G13Button::UNDEF3 => 36,
            G13Button::LIGHT => 37,
            G13Button::LIGHT2 => 38,
            G13Button::MISCTOGGLE => 39,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum G13KeyPress {
    Pressed,
    Released
}

impl From<bool> for G13KeyPress {
    fn from(b: bool) -> Self {
        if b { G13KeyPress::Pressed } else { G13KeyPress::Released }
    }
}

pub type G13KeyEvent = (G13Button, G13KeyPress);

#[derive(Debug)]
pub enum G13Error {
    UInputError(uinput::Error),
    USBError(libusb::Error),
}

impl From<uinput::Error> for G13Error {
    fn from(err: uinput::Error) -> Self {
        G13Error::UInputError(err)
    }
}

impl From<libusb::Error> for G13Error {
    fn from(err: libusb::Error) -> Self {
        G13Error::USBError(err)
    }
}

#[derive(Clone, Debug)]
pub enum G13KeyAction<T=uinput::event::keyboard::Key>
        where T: uinput::event::Press + uinput::event::Release + Copy + Clone {
    Noop,
    Key(T),
    MultipleKeys(Vec<T>),
}

impl<T> G13KeyAction<T> where T: uinput::event::Press + uinput::event::Release + Copy {
    pub fn pressed(&self, device: &mut G13Device) -> Result<(), G13Error> {
        match self {
            G13KeyAction::Noop => { Ok(()) },
            G13KeyAction::Key(_key) => { Ok(()) },
            G13KeyAction::MultipleKeys(ref _keys) => { Ok(()) }
        }
    }

    pub fn released(&self, device: &mut G13Device) -> Result<(), G13Error> {
        match self {
            G13KeyAction::Noop => { Ok(()) },
            G13KeyAction::Key(_key) => { Ok(()) },
            G13KeyAction::MultipleKeys(_) => { Ok(()) }
        }
    }
}

