use std::ffi::CString;
use std::os::raw::c_char;

extern crate rust_dxlib;
use rust_dxlib::*;

type KeyStateBuff = [c_char; 256];

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

    pub fn get_key_state() -> KeyStateBuff {
        unsafe {
            let mut buf: KeyStateBuff;
            dx_GetHitKeyStateAll(buf);
            return buf;
        }
    }

    pub fn set(&mut self, buffer: &KeyStateBuff) {
        self.up = if buffer[KEY_INPUT_UP as usize] == 0 {
            false
        } else {
            true
        };
        self.down = if buffer[KEY_INPUT_DOWN as usize] == 0 {
            false
        } else {
            true
        };
        self.left = if buffer[KEY_INPUT_LEFT as usize] == 0 {
            false
        } else {
            true
        };
        self.right = if buffer[KEY_INPUT_RIGHT as usize] == 0 {
            false
        } else {
            true
        };
        self.z = if buffer[KEY_INPUT_Z as usize] == 0 {
            false
        } else {
            true
        };
        self.x = if buffer[KEY_INPUT_X as usize] == 0 {
            false
        } else {
            true
        };
        self.c = if buffer[KEY_INPUT_C as usize] == 0 {
            false
        } else {
            true
        };
        self.shift = if buffer[KEY_INPUT_LSHIFT as usize] == 0 {
            false
        } else {
            true
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
