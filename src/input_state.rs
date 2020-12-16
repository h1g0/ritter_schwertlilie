extern crate piston_window;
use piston_window::*;

struct InputState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
impl InputState {
    fn new() -> InputState {
        return InputState {
            up: false,
            down: false,
            left: false,
            right: false,
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
            _ => {}
        }
    }
}
