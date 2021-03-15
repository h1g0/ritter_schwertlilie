use crate::math::rectangle::*;

/// 画像を管理するクラス
pub struct Image{
    pub size: Rectangle,
    pub image: u32,
}

impl Image{
    pub new(size: Rectangle, image: u32)->Image{
        Image{
            size,
            image,
        }
    }
}