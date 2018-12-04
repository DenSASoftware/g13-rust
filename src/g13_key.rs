#[derive(Copy, Clone)]
pub struct G13Key {
    pub is_pressed: bool
}

impl G13Key {
    pub fn new() -> Self {
        G13Key { is_pressed: false }
    }

    pub fn pressed(&self, key_name: &str) {
        info!("Key {} got pressed", key_name);
    }

    pub fn released(&self, key_name: &str) {
        info!("Key {} got released", key_name);
    }
}

impl Default for G13Key {
    fn default() -> Self {
        G13Key { is_pressed: false }
    }
}

pub struct G13KeyInformation {
    pub index: u8,
    pub name: &'static str
}

pub const G13_KEYS: [G13KeyInformation; 40] = [
    G13KeyInformation { index: 0, name: "G1" },
    G13KeyInformation { index: 1, name: "G2" },
    G13KeyInformation { index: 2, name: "G3" },
    G13KeyInformation { index: 3, name: "G4" },
    G13KeyInformation { index: 4, name: "G5" },
    G13KeyInformation { index: 5, name: "G6" },
    G13KeyInformation { index: 6, name: "G7" },
    G13KeyInformation { index: 7, name: "G8" },
    G13KeyInformation { index: 8, name: "G9" },
    G13KeyInformation { index: 9, name: "G10" },
    G13KeyInformation { index: 10, name: "G11" },
    G13KeyInformation { index: 11, name: "G12" },
    G13KeyInformation { index: 12, name: "G13" },
    G13KeyInformation { index: 13, name: "G14" },
    G13KeyInformation { index: 14, name: "G15" },
    G13KeyInformation { index: 15, name: "G16" },
    G13KeyInformation { index: 16, name: "G17" },
    G13KeyInformation { index: 17, name: "G18" },
    G13KeyInformation { index: 18, name: "G19" },
    G13KeyInformation { index: 19, name: "G20" },
    G13KeyInformation { index: 20, name: "G21" },
    G13KeyInformation { index: 21, name: "G22" },
    G13KeyInformation { index: 22, name: "UNDEF1" },
    G13KeyInformation { index: 23, name: "LIGHT_STATE" },
    G13KeyInformation { index: 24, name: "BD" },
    G13KeyInformation { index: 25, name: "L1" },
    G13KeyInformation { index: 26, name: "L2" },
    G13KeyInformation { index: 27, name: "L3" },
    G13KeyInformation { index: 28, name: "L4" },
    G13KeyInformation { index: 29, name: "M1" },
    G13KeyInformation { index: 30, name: "M2" },
    G13KeyInformation { index: 31, name: "M3" },
    G13KeyInformation { index: 32, name: "MR" },
    G13KeyInformation { index: 33, name: "LEFT" },
    G13KeyInformation { index: 34, name: "DOWN" },
    G13KeyInformation { index: 35, name: "TOP" },
    G13KeyInformation { index: 36, name: "UNDEF3" },
    G13KeyInformation { index: 37, name: "LIGHT" },
    G13KeyInformation { index: 38, name: "LIGHT2" },
    G13KeyInformation { index: 39, name: "MISC_TOGGLE" },
];
