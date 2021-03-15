use crate::math::rectangle::*;
use crate::math::vec2d::*;

/// 背景クラス
pub struct Background {
    /// ゲーム上で使用されるフィールドの大きさ
    pub field_size: Vec2d<u32>,
    /// 実際に描画される視野の大きさ
    pub view_size: Vec2d<u32>,
    /// 視野の中心点
    pub center_point: Vec2d<i32>,
}

impl Background {
    pub fn new(
        field_size: Vec2d<u32>,
        view_size: Vec2d<u32>,
        center_point: Vec2d<i32>,
    ) -> Background {
        Background {
            field_size,
            view_size,
            center_point,
        }
    }
    /// 現在の中心点を元にフィールド内での視野の座標を求める
    pub fn get_view_rect(&mut self) -> Rectangle {
        let half_show_x: i32 = (self.view_size.x / 2) as i32;
        let half_show_y: i32 = (self.view_size.y / 2) as i32;

        unimplemented!();
        // TODO: ほろ酔いで実装したので下のは色々おかしい
        /*
        Rectangle::new(
            Vec2d::<i32>::new(
                if self.center_point.x <= half_show_x {
                    // 視界の左端がフィールド左端よりも左
                    0
                } else {
                    if self.center_point.x + half_show_x > self.field_size.x as i32 {
                        //視界の右端がフィールド右端よりも右
                        self.field_size.x as i32 - self.view_size.x as i32
                    } else {
                        //視界の左右がフィールド内
                        self.center_point.x - half_show_x
                    }
                },
                if self.center_point.y <= half_show_y {
                    //視界の上端がフィールド上端よりも上
                    0
                } else {
                    if self.center_point.y + half_show_y > self.field_size.y as i32 {
                        //視界の下端がフィールド下端よりも下
                        self.field_size.y as i32 - self.view_size.y as i32
                    } else {
                        //視界の上下がフィールド内
                        self.center_point.y - half_show_y
                    }
                },
            ),
            Vec2d::<i32>::new(self.field_size.x as i32, self.field_size.y as i32),
        )
        */
    }
    pub fn set_center_point(&mut self, p: Vec2d<i32>) {
        self.center_point = p;
    }

    pub fn draw() {
        unimplemented!();
    }
}
