use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// 角度の単位（ラジアン `RAD` もしくは度 `DEG`）
pub enum Unit {
    RAD,
    DEG,
}
/// 角度
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle {
    rad: f64,
}
impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            rad: Angle::revise(-self.rad),
        }
    }
}
impl Add for Angle {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            rad: Angle::revise(self.rad + other.rad),
        }
    }
}
impl AddAssign for Angle {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            rad: Angle::revise(self.rad + other.rad),
        }
    }
}
impl Sub for Angle {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            rad: Angle::revise(self.rad - other.rad),
        }
    }
}
impl SubAssign for Angle {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            rad: Angle::revise(self.rad - other.rad),
        }
    }
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
    /// `Angle`を新規作成する
    pub fn new(angle: f64, unit: Unit) -> Angle {
        let mut result = Angle { rad: 0.0 };
        result.set(angle, unit);
        return result;
    }
    /// 角度を指定して代入する
    pub fn set(&mut self, angle: f64, unit: Unit) {
        self.rad = if unit == Unit::DEG {
            Angle::revise(angle * Angle::DEG_TO_RAD)
        } else {
            Angle::revise(angle)
        };
    }
    /// ラジアンを求める
    #[inline]
    pub fn to_rad(&self) -> f64 {
        self.rad
    }
    ///ディグリーを求める
    #[inline]
    pub fn to_deg(&self) -> f64 {
        self.rad * Angle::RAD_TO_DEG
    }

    #[inline]
    pub fn sin(&self) -> f64 {
        self.rad.sin()
    }
    #[inline]
    pub fn cos(&self) -> f64 {
        self.rad.cos()
    }

    #[inline]
    pub fn vx(&self, scalar: f64) -> f64 {
        self.cos() * scalar
    }
    #[inline]
    pub fn vy(&self, scalar: f64) -> f64 {
        self.sin() * scalar
    }
}
#[inline]
pub fn rad(angle: f64) -> Angle {
    Angle::new(angle, Unit::RAD)
}
#[inline]
pub fn deg(angle: f64) -> Angle {
    Angle::new(angle, Unit::DEG)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn get_deg_test() {
        assert_approx_eq!(deg(180.0).to_deg(), 180.0);
    }
    #[test]
    fn get_deg_test2() {
        assert_approx_eq!(deg(90.0).to_deg(), 90.0);
    }
    #[test]
    fn get_deg_test3() {
        assert_approx_eq!(deg(360.0).to_deg(), 0.0);
    }
    #[test]
    fn get_rad_test() {
        assert_approx_eq!(deg(180.0).to_rad(), std::f64::consts::PI);
    }
    #[test]
    fn get_rad_test2() {
        assert_approx_eq!(deg(90.0).to_rad(), std::f64::consts::PI / 2.0);
    }
    #[test]
    fn deg_and_rad_test() {
        assert_approx_eq!(deg(180.0).to_rad(), rad(std::f64::consts::PI).to_rad());
    }
    #[test]
    fn overflowed_angle_test() {
        assert_approx_eq!(deg(180.0 + 360.0).to_rad(), deg(180.0).to_rad());
    }
    #[test]
    fn negative_angle_test() {
        assert_approx_eq!(deg(-180.0).to_rad(), deg(180.0).to_rad());
    }

    #[test]
    fn add_test() {
        assert_approx_eq!((deg(315.0) + deg(90.0)).to_rad(), deg(45.0).to_rad());
    }
    #[test]
    fn sub_test() {
        assert_approx_eq!((deg(45.0) - deg(90.0)).to_rad(), deg(315.0).to_rad());
    }
    #[test]
    fn sub_test2() {
        assert_approx_eq!((deg(45.0) - deg(315.0)).to_rad(), deg(90.0).to_rad());
    }
    #[test]
    fn sub_test3() {
        assert_approx_eq!((deg(315.0) - deg(45.0)).to_rad(), deg(270.0).to_rad());
    }
}
