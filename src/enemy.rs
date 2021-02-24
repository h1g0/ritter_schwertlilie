extern crate rust_dxlib;
use rust_dxlib::*;

use crate::input_state::*;
use crate::math::angle::*;
use crate::math::vec2d::*;

/// 敵
pub struct Enemy {
    /// 敵の画像
    pub texture: i32,
    /// 画像サイズ
    pub texture_size: Vec2d<u32>,
    /// 当たり判定サイズ
    pub hit_size: Vec2d<u32>,
    /// 座標
    pub pos: Vec2d<f64>,
    /// 移動角度
    pub angle: Angle,
    /// 移動速度
    pub speed: f64,
    /// 敵のHP
    pub life: u32,
}

impl Enemy {
    pub fn new(
        texture: i32,
        texture_size: Vec2d<u32>,
        hit_size: Vec2d<u32>,
        pos: Vec2d<u32>,
        v: Vec2d<f64>,
        life: u32,
    ) -> Enemy {
        return Enemy {
            texture,
            texture_size,
            hit_size,
            pos,
            vxy,
            life,
        };
    }
    /// 位置を設定する
    pub fn set_pos(&mut self, pos: &Vec2d<u32>) {
        self.pos = pos;
    }

    /// 設定された角度とスピードを元に移動する
    pub fn move_self(&mut self) {
        let v = Vec2d::<f64>::new(self.angle.cos() * self.speed, self.angle.sin() * self.speed);
        self.pos += v;
    }
    /// HPを設定する
    pub fn set_life(&mut self, life: u32) {
        self.life = life;
    }
    /// 敵にダメージを与える
    pub fn set_damage(&mut self, damage: u32) {
        self.life += damage;
    }
    /// 敵を描画する
    pub fn draw() {
        unimplemented!();
    }
}
