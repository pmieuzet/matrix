use std::ops::{Add, Div, Mul, Sub};

pub trait RealNumber: PartialEq + Copy + Mul + Add + Div + Sub {
    fn abs(self) -> f32;
    fn zero() -> Self;
    fn one() -> Self;
    fn default() -> Self;
}
impl RealNumber for f32 {
    fn abs(self) -> f32 {
        self.abs()
    }
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn default() -> Self {
        Default::default()
    }
}