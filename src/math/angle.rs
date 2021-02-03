#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Unit {
    RAD,
    DEG,
}

#[derive(Debug, Clone, Copy,PartialEq)]
pub struct Angle {
    rad: f64,
}

impl Angle {
    /// 最小値（この値を含む）: 0
    const MIN_INCLUSIVE: f64 = 0.0;
    /// 最大値（この値を含まない）：2π
    const MAX_EXCLUSIVE: f64 = std::f64::consts::PI * 2.0;

    const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
    const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;

    /// 角度が上限値下限値を超えていたら補正した値を返す
    fn revise(rad: f64) -> f64 {
        let mut r = rad;
        while r < Angle::MIN_INCLUSIVE {
            r += Angle::MAX_EXCLUSIVE;
        }
        while r >= Angle::MAX_EXCLUSIVE {
            r -= Angle::MAX_EXCLUSIVE;
        }
        return r;
    }

    pub fn new(angle: f64,unit:Unit)->Angle{
        let mut result = Angle{
            rad: 0.0,
        };
        result.set(angle,unit);
        return result;
    }

    pub fn set(&mut self, angle: f64, unit: Unit) {
        self.rad = if unit == Unit::DEG {
            Angle::revise(angle * Angle::DEG_TO_RAD)
        } else {
            Angle::revise(angle)
        };
    }

    pub fn rad(&self)->f64{
        return self.rad;
    }

    pub fn deg(&self)->f64{
        return self.rad * Angle::RAD_TO_DEG;
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deg_to_rad_test() {
        assert_eq!(
            Angle::new(180.0, Unit::DEG),
            Angle::new(std::f64::consts::PI, Unit::RAD)
        );
    }
}
