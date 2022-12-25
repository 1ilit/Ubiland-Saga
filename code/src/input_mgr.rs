use glium::glutin::event::{ElementState, VirtualKeyCode};

pub struct InputManager {
    keys: [bool; 163],
    prev_keys: [bool; 163],
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            keys: [false; 163],
            prev_keys: [false; 163],
        }
    }

    fn key_changed(&mut self, key: VirtualKeyCode) -> bool {
        let temp = self.prev_keys[key as usize];
        self.prev_keys[key as usize] = false;
        temp
    }

    pub fn key_went_down(&mut self, key: VirtualKeyCode) -> bool {
        self.keys[key as usize] && self.key_changed(key)
    }

    pub fn key_went_up(&mut self, key: VirtualKeyCode) -> bool {
        !self.keys[key as usize] && self.key_changed(key)
    }

    pub fn update(&mut self, state: ElementState, key: VirtualKeyCode) {
        if state != ElementState::Released {
            if !self.keys[key as usize] {
                self.keys[key as usize] = true;
            }
        } else {
            self.keys[key as usize] = false;
        }

        self.prev_keys[key as usize] = state != ElementState::Pressed;
    }
}
