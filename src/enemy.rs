extern crate piston_window;
use piston_window::*;

use crate::input_state::*;

pub struct Enemy{
    /// 自機の画像
    pub texture: G2dTexture,
    /// 画像サイズ
    pub texture_size: Size,
    /// 当たり判定サイズ
    pub hit_size: Size,
    /// X座標
    pub x: f64,
    /// Y座標
    pub y: f64,
    /// X方向の移動量
    pub vx: f64,
    /// Y方向の移動量
    pub vy:f64,
    /// 敵のHP
    pub life: u32,
}

impl Enemy{
    pub fn new(
        texture: G2dTexture,
        texture_size: Size,
        hit_size: Size,
        x: f64,
        y: f64,
        vx: f64,
        vy: f64,
        life: u32,)->Enemy{
        return Enemy{
            texture,
            texture_size,
            hit_size,
            x,
            y,
            vx,
            vy,
            life,
        }
    }
    pub fn draw(){
        unimplemented!();
    }
    pub fn get_pos(&self)->(f64,f64){
        return (self.x,self.y);
    }

}