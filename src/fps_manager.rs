pub struct FpsManager {
    fix_fps: bool,
    default_fps: i32,

    fps: i32,
    counter: i32,
    old_time: i32,
}

impl FpsManager {
    pub fn new(fix_fps: bool, default_fps: i32, now_time: i32) -> FpsManager {
        return FpsManager {
            fix_fps,
            default_fps,
            fps: default_fps,
            counter: 0,
            old_time: now_time,
        };
    }

    pub fn fix(&self,) {
        if !self.fix_fps {return};

    }

    /// FPSを測定する
    pub fn measure(&mut self, now_time: i32) -> i32 {
        self.counter += 1;
        if now_time - self.old_time >= 1000 {
            let diff = (now_time - self.old_time) - 1000; //誤差差分
            self.old_time = now_time - diff; //誤差修正
            self.fps = self.counter; //fpsを保存
            self.counter = 0; //カウントリセット
        }
        return self.fps;
    }

    /// 保存されている最新のFPSを返す
    pub fn get(&self)->i32{
        return self.fps;
    }

    /// 最新のFPSのデフォルトFPSに対する割合を返す
    pub fn get_percent(&mut self) -> f32{
        return self.fps as f32 / self.default_fps as f32;
    }
}
