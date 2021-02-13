use crate::math::point2d::*;

pub struct Rectangle{
    start: Point2d,
    end: Point2d,
}

impl Rectangle{
    pub fn new(start: Point2d, end: Point2d)->Rectangle{
        Rectangle{start,end}
    }

    pub fn from_wh(width: Point,height:Point)->Rectangle{
        Rectangle{
            start:Point2d::new(0,0),
            end:Point2d::new(width,height),
        }
    }

    pub fn width(&self)->u32{
        self.end.x - self.start.x
    }
    pub fn height(&self)->u32{
        self.end.y - self.start.y
    }
}