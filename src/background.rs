use crate::math::vec2d::*;

pub struct Background {
    field_size: Vec2d<u32>,
    show_size: Vec2d<u32>,
    center_point: Vec2d<i32>,
}

impl Background {
    pub fn new(
        field_size: Vec2d<u32>,
        show_size: Vec2d<u32>,
        center_point: Vec2d<i32>,
    ) -> Background {
        Background {
            field_size,
            show_size,
            center_point,
        }
    }

    pub fn set_center_point(&mut self, p: Vec2d<i32>) {
        self.center_point = p;
    }

    pub fn draw() {
        unimplemented!();
    }
}
