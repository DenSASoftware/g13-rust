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

pub const G13_KEYS: [G13KeyInformation; 22] = [
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
];

