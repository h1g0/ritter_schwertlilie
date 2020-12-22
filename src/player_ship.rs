extern crate piston_window;
use piston_window::*;

use crate::input_state::*;

pub struct PlayerShip {
    /// 自機の可動範囲
    field_size: Size,
    /// 自機の画像サイズ
    image_size: Size,
    /// 自機の当たり判定サイズ
    hit_size: Size,
    /// 自機のY座標
    x: f64,
    /// 自機のY座標
    y: f64,
    /// 自機の残機
    life: u32,
}
impl PlayerShip {
    const SPEED: f64 = 2.0;

    pub fn new(
        field_size: Size,
        image_size: Size,
        hit_size: Size,
        x: f64,
        y: f64,
        life: u32,
    ) -> PlayerShip {
        let mut result: PlayerShip = PlayerShip {
            field_size,
            image_size,
            hit_size,
            x: 0.0,
            y: 0.0,
            life,
        };
        result.set_pos(x, y);
        return result;
    }
    pub fn get_pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }
    pub fn set_pos(&mut self, x: f64, y: f64) -> (f64, f64) {
        self.x = x;
        self.y = y;
        (self.x, self.y)
    }
    pub fn move_by(&mut self, vx: f64, vy: f64) -> (f64, f64) {
        self.set_pos(self.x + vx, self.y + vy)
    }
    pub fn move_by_input(&mut self, input: &InputState) {
        let vx = if input.left {
            -PlayerShip::SPEED
        } else if input.right {
            PlayerShip::SPEED
        } else {
            0.0
        };
        let vy = if input.up {
            -PlayerShip::SPEED
        } else if input.down {
            PlayerShip::SPEED
        } else {
            0.0
        };

        self.move_by(vx, vy);
    }
}
