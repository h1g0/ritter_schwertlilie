extern crate piston_window;
use piston_window::*;

use crate::input_state::*;

pub struct Enemy{
    /// 画像サイズ
    image_size: Size,
    /// 当たり判定サイズ
    hit_size: Size,
    /// X座標
    x: f64,
    /// Y座標
    y: f64,
    /// X方向の移動量
    vx: f64,
    /// Y方向の移動量
    vy:f64,
    /// 敵のHP
    life: u32,
}

impl Enemy{
    pub fn new()->Enemy{
        unimplemented!();
    }
    pub fn draw(){
        unimplemented!();
    }
    pub fn get_pos(&self)->(f64,f64){
        return (self.x,self.y);
    }

}