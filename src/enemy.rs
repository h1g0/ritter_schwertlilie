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
    /// 自機の残機
    life: u32,
}