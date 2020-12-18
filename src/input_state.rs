extern crate piston_window;
use piston_window::*;

struct InputState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    z:bool,
    x:bool,
    c:bool,
    shift:bool,
}
impl InputState {
    fn new() -> InputState {
        return InputState {
            up: false,
            down: false,
            left: false,
            right: false,
            z:false,
            x:false,
            c:false,
            shift:false,
        };
    }

    fn set(&mut self, key: &ButtonArgs) {
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
}
