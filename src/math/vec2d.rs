use std::ops::{Add, Sub};
/// 2次元ベクトル
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Vec2d<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Vec2d<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
/*
impl<T> AddAssign for Vec2d<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
*/
impl<T: Sub<Output = T>> Sub for Vec2d<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T> Vec2d<T> {
    pub fn new(x: T, y: T) -> Vec2d<T> {
        Vec2d { x, y }
    }
    pub fn from_tuple(t: (T, T)) -> Vec2d<T> {
        Vec2d::new(t.0, t.1)
    }
}
