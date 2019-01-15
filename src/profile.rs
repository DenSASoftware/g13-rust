use crate::key::{G13KeyAction, G13Button};
use crate::constants::G13_KEYS_LENGTH;

use log::info;

type KeyBindings = [G13KeyAction; G13_KEYS_LENGTH];

pub struct G13KeyProfile {
    binding_pages: [KeyBindings; 3],
    active_binding: usize,
}

fn empty_pages() -> [[G13KeyAction; G13_KEYS_LENGTH]; 3] {
    unsafe {
        let mut arr: [[G13KeyAction; G13_KEYS_LENGTH]; 3] = std::mem::uninitialized();
        for i in 0..3 {
            for x in 0..G13_KEYS_LENGTH {
                arr[i][x] = G13KeyAction::Noop;
            }
        }

        arr
    }
}

impl G13KeyProfile {
    pub fn new() -> Self {
        Self {
            binding_pages: empty_pages(),
            active_binding: 0,
        }
    }

    pub fn get_action(&self, key: G13Button) -> &G13KeyAction {
        &self.binding_pages[self.active_binding][usize::from(key)]
    }

    pub fn set_page(&mut self, page: usize) {
        if page < 3 {
            info!("Switching to page {}", page);
            self.active_binding = page;
        }
    }
}

