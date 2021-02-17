use crate::math::vec2d::*;
use crate::math::rectangle::*;

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

    pub fn get_show_rect(&mut self)->Rectangle{
        let half_show_x: i32 = (self.show_size.x / 2) as i32;
        let half_show_y: i32 = (self.show_size.y / 2) as i32;
        Rectangle::new(
            Vec2d::<i32>::new(
                // TODO: x,yがfield_sizeを超える時の処理
            if self.center_point.x > half_show_x{
                if self.center_point.x + half_show_x > self.field_size.x{
                    self.
                }
                self.center_point.x - half_show_x
            }else{
                0
            },
            if self.center_point.y > half_show_y{
                self.center_point.y - half_show_y
            }else{
                0
            }),
            Vec2d::<i32>::new(self.field_size.x as i32,self.field_size.y as i32)
        )
        
    }
    pub fn set_center_point(&mut self, p: Vec2d<i32>) {
        self.center_point = p;
    }

    pub fn draw() {
        unimplemented!();
    }
}
