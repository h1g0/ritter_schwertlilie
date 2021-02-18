use crate::math::vec2d::*;

/// 矩形
pub struct Rectangle {
    /// 左上の座標
    start: Vec2d<i32>,
    /// 右下の座標
    end: Vec2d<i32>,
}

impl Rectangle {
    /// 左上の座標と矩形の大きさから`Rectangle`を生成する
    pub fn new(start: Vec2d<i32>, size: Vec2d<i32>) -> Rectangle {
        Rectangle {
            start,
            end: start + size,
        }
    }
    /// 左上と右下の座標から`Rectangle`を生成する
    pub fn from_start_end(start: Vec2d<i32>, end: Vec2d<i32>) -> Rectangle {
        Rectangle { start, end }
    }

    pub fn size(&self) -> Vec2d<i32> {
        self.end - self.start
    }
    /// 左辺のx座標
    pub fn left_x(&self) -> i32 {
        self.start.x
    }
    /// 右辺のx座標
    pub fn right_x(&self) -> i32 {
        self.end.x
    }
    /// 上辺のy座標
    pub fn up_y(&self) -> i32 {
        self.start.y
    }
    /// 下辺のy座標
    pub fn bottom_y(&self) -> i32 {
        self.end.y
    }
    /// 左上の座標
    pub fn left_up(&self) -> Vec2d<i32> {
        Vec2d::new(self.left_x(), self.up_y())
    }
    /// 左下の座標
    pub fn left_bottom(&self) -> Vec2d<i32> {
        Vec2d::new(self.left_x(), self.bottom_y())
    }
    /// 右上の座標
    pub fn right_up(&self) -> Vec2d<i32> {
        Vec2d::new(self.right_x(), self.up_y())
    }
    /// 右下の座標
    pub fn right_bottom(&self) -> Vec2d<i32> {
        Vec2d::new(self.right_x(), self.bottom_y())
    }
}
