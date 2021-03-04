extern crate rust_dxlib;
use rust_dxlib::*;

use crate::input_state::*;
use crate::math::angle::*;
use crate::math::vec2d::*;

/// 自機
pub struct PlayerShip {
    /// 自機の可動範囲
    pub field_size: Vec2d<i32>,
    /// 自機の画像
    pub texture: i32,
    /// 自機の画像サイズ
    pub texture_size: Vec2d<i32>,
    /// 自機の当たり判定サイズ
    pub hit_size: Vec2d<i32>,
    /// 自機の座標
    pub pos: Vec2d<f64>,
    /// 自機の角度
    pub angle: Angle,
    /// 自機の目標とする角度
    dist_angle: Angle,
    /// 自機の残機
    pub life: u32,
}
impl PlayerShip {
    const SPEED: f64 = 2.0;

    pub fn new(
        field_size: Vec2d<i32>,
        texture: i32,
        texture_size: Vec2d<i32>,
        hit_size: Vec2d<i32>,
        pos: Vec2d<f64>,
        life: u32,
    ) -> PlayerShip {
        let mut result: PlayerShip = PlayerShip {
            field_size,
            texture,
            texture_size,
            hit_size,
            pos,
            angle: Angle::new(270.0, Unit::DEG),
            dist_angle: Angle::new(270.0, Unit::DEG),
            life,
        };
        result.set_pos(pos);
        return result;
    }
    /*
    pub fn get_pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }
    */
    pub fn set_pos(&mut self, coord: Vec2d<f64>) -> Vec2d<f64> {
        self.pos.x = coord.x;
        self.pos.y = coord.y;
        self.pos
    }
    pub fn move_by(&mut self, vxy: Vec2d<f64>) -> Vec2d<f64> {
        self.set_pos(self.pos + vxy)
    }
    fn get_dist_angle_by_input(input: &InputState) -> Option<Angle> {
        if input.down && input.right {
            Some(deg(45.0))
        } else if input.down && input.left {
            Some(deg(135.0))
        } else if input.up && input.left {
            Some(deg(225.0))
        } else if input.up && input.right {
            Some(deg(315.0))
        } else if input.up && input.down {
            None
        } else if input.left && input.right {
            None
        } else if input.right {
            Some(deg(0.0))
        } else if input.down {
            Some(deg(90.0))
        } else if input.left {
            Some(deg(180.0))
        } else if input.up {
            Some(deg(270.0))
        } else {
            None
        }
    }
    fn rotate(&mut self) {
        let max = deg(24.0);
        if self.dist_angle - self.angle <= self.angle - self.dist_angle {
            //目標が右側（含む真後ろ）にある
            if f64::abs(self.dist_angle.to_rad() - self.angle.to_rad()) <= max.to_rad() {
                self.angle = self.dist_angle;
            } else {
                self.angle += max;
            }
        } else {
            // 目標が左側にある
            if f64::abs(self.dist_angle.to_rad() - self.angle.to_rad()) <= max.to_rad() {
                self.angle = self.dist_angle;
            } else {
                self.angle -= max;
            }
        }
    }
    pub fn move_by_input(&mut self, input: &InputState) {
        let speed = if input.shift {
            PlayerShip::SPEED * 2.0
        } else {
            PlayerShip::SPEED
        };
        if let Some(dist) = PlayerShip::get_dist_angle_by_input(input) {
            let vxy = Vec2d::new(dist.cos() * speed, dist.sin() * speed);
            self.move_by(vxy);
            self.dist_angle = dist;
        }
        self.rotate();
    }
    pub fn draw(&self) {
        //描画位置とサイズをセット
        //指定した描画位置は画像の左上になるので、中央に配置する
        unsafe {
            dx_SetDrawBlendMode(DX_BLENDMODE_ADD, 255);
            dx_DrawRotaGraph(
                self.pos.x as i32,
                self.pos.y as i32,
                1.0,
                self.angle.to_rad(),
                self.texture,
                TRUE,
                FALSE,
            );
        }
    }
}
