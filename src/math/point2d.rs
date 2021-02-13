pub type Point = u32;

pub struct Point2d {
    pub x: Point,
    pub y: Point,
}
impl Point2d {
    pub fn new(x: Point, y: Point) -> Point2d {
        Point2d { x, y }
    }
}
