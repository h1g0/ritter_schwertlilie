extern crate rust_dxlib;
use rust_dxlib::*;

pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub z: bool,
    pub x: bool,
    pub c: bool,
    pub shift: bool,
}
impl InputState {
    pub fn new() -> InputState {
        return InputState {
            up: false,
            down: false,
            left: false,
            right: false,
            z: false,
            x: false,
            c: false,
            shift: false,
        };
    }
    /*
    pub fn set(&mut self, key: &ButtonArgs) {
        match key.button {
            Button::Keyboard(Key::Up) => {
                self.up = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::Down) => {
                self.down = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::Left) => {
                self.left = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::Right) => {
                self.right = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::Z) => {
                self.z = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::X) => {
                self.x = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::C) => {
                self.c = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::LShift) => {
                self.shift = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            _ => {}
        }
    }
    */
}
