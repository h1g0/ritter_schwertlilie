

pub struct Background {
    field_size: (u32, u32),
    show_size: (u32, u32),
    center_point: (u32, u32),
}

impl Background {
    pub fn new(
        field_size: (u32, u32),
        show_size: (u32, u32),
        center_point: (u32, u32),
    ) -> Background {
        Background {
            field_size,
            show_size,
            center_point,
        }
    }

    pub fn set_center_point(&mut self, p: (u32,u32)){
        self.center_point = p;
    }

    pub fn draw(){
        unimplemented!();
    }
}
