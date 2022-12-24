use glium::glutin::event::{ElementState, VirtualKeyCode};

pub enum Direction {
    UP = 0,
    DOWN,
    LEFT,
    RIGHT,
    SPACE,
}

pub struct InputManager {
    keys: [bool; 5],
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            keys: [false, false, false, false, false],
        }
    }

    pub fn dir_is_pressed(&self, key: Direction) -> bool {
        self.keys[key as usize]
    }

    pub fn update(&mut self, state: ElementState, key: Option<VirtualKeyCode>) {
        let is_down = if state == ElementState::Released {
            false
        } else {
            true
        };
        match key {
            Some(VirtualKeyCode::Up) => self.keys[0] = is_down,
            Some(VirtualKeyCode::Down) => self.keys[1] = is_down,
            Some(VirtualKeyCode::Left) => self.keys[2] = is_down,
            Some(VirtualKeyCode::Right) => self.keys[3] = is_down,
            Some(VirtualKeyCode::Space) => self.keys[4] = is_down,
            _ => return,
        }
    }
}
