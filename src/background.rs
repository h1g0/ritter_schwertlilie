pub struct BackGround{
    field_size: (u32,u32),
    show_size: (u32,u32),
    center_point: (u32,u32),
}

impl BackGround{
    pub fn new(field_size:(u32,u32),show_size:(u32,u32),center_point:(u32,u32))->BackGround{
        BackGround{
            field_size,
            show_size,
            center_point,
        }
    }
    
}