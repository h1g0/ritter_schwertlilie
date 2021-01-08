extern crate rust_dxlib;
use rust_dxlib::*;

use crate::input_state::*;

pub struct PlayerShip {
    /// 自機の可動範囲
    pub field_size: (i32,i32),
    /// 自機の画像
    pub texture: i32,
    /// 自機の画像サイズ
    pub texture_size: (i32,i32),
    /// 自機の当たり判定サイズ
    pub hit_size: (i32,i32),
    /// 自機のX座標
    pub x: f64,
    /// 自機のY座標
    pub y: f64,
    /// 自機の残機
    pub life: u32,
}
impl PlayerShip {
    const SPEED: f64 = 2.0;

    pub fn new(
        field_size: (i32,i32),
        texture: i32,
        texture_size: (i32,i32),
        hit_size: (i32,i32),
        x: f64,
        y: f64,
        life: u32,
    ) -> PlayerShip {
        let mut result: PlayerShip = PlayerShip {
            field_size,
            texture,
            texture_size,
            hit_size,
            x: 0.0,
            y: 0.0,
            life,
        };
        result.set_pos(x, y);
        return result;
    }
    /*
    pub fn get_pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }
    */
    pub fn set_pos(&mut self, x: f64, y: f64) -> (f64, f64) {
        self.x = x;
        self.y = y;
        (self.x, self.y)
    }
    pub fn move_by(&mut self, vx: f64, vy: f64) -> (f64, f64) {
        self.set_pos(self.x + vx, self.y + vy)
    }
    pub fn move_by_input(&mut self, input: &InputState) {
        let speed = if input.shift{
            PlayerShip::SPEED * 2.0
        }else{
            PlayerShip::SPEED
        };
        let vx = if input.left && self.x > (self.texture_size.0 / 2) as f64 {
            -speed
        } else if input.right && self.x < (self.field_size.0 - self.texture_size.0 / 2) as f64 {
            speed
        } else {
            0.0
        };
        let vy = if input.up && self.y > (self.texture_size.1 / 2) as f64 {
            -speed
        } else if input.down && self.y < (self.field_size.1 - self.texture_size.1 / 2) as f64 {
            speed
        } else {
            0.0
        };
        self.move_by(vx, vy);
    }
    pub fn draw(&self) {
        //描画位置とサイズをセット
        //指定した描画位置は画像の左上になるので、中央に配置する
        unsafe{
            dx_SetDrawBlendMode(DX_BLENDMODE_ALPHA,255);
            dx_DrawGraph(self.x as i32, self.y as i32, self.texture, TRUE);
        }
    }
}
