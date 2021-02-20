extern crate rust_dxlib;
use rust_dxlib::*;

use crate::input_state::*;
use crate::math::angle::*;
use crate::math::vec2d::*;

pub struct Enemy {
    /// 敵の画像
    pub texture: i32,
    /// 画像サイズ
    pub texture_size: Vec2d<u32>,
    /// 当たり判定サイズ
    pub hit_size: Vec2d<u32>,
    /// 座標
    pub pos: Vec2d<f64>,
    /// 移動量
    pub vxy: Vec2d<f64>,
    /// 敵のHP
    pub life: u32,
}

impl Enemy {
    pub fn new(
        texture: i32,
        texture_size: Vec2d<u32>,
        hit_size: Vec2d<u32>,
        pos: Vec2d<u32>,
        vxy: Vec2d<f64>,
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
    pub fn move_by(&mut self,pos: &Vec2d<u32>){
        self.pos = pos;
    }
    /// 移動ベクトルを設定する
    pub fn set_vxy(&mut self, vxy = &Vec2d<f64>){
        self.vxy = vxy;
    }
    /// 設定された移動ベクトルを元に移動する
    pub fn move_by_vxy(&mut self){
        self.pos +=  self.vxy;
    }
    pub fn draw() {
        unimplemented!();
    }
}
