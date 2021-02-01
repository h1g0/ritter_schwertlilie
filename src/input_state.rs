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
    pub esc: bool,
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
            esc: false,
        };
    }

    pub fn get_key_state() -> KeyStateBuff {
        unsafe {
            let mut buf: KeyStateBuff = [0; 256];
            dx_GetHitKeyStateAll(buf.as_mut_ptr());
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
        self.esc = if buffer[KEY_INPUT_ESCAPE as usize] == 0 {
            false
        } else {
            true
        };
    }
}
