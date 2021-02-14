use crate::math::vec2d::*;
pub struct Rectangle {
    start: Vec2d<i32>,
    end: Vec2d<i32>,
}

impl Rectangle {
    pub fn new(start: Vec2d<i32>, size: Vec2d<i32>) -> Rectangle {
        Rectangle {
            start,
            end: start + size,
        }
    }

    pub fn from_start_end(start: Vec2d<i32>, end: Vec2d<i32>) -> Rectangle {
        Rectangle { start, end }
    }

    pub fn size(&self) -> Vec2d<i32> {
        self.end - self.start
    }
}
