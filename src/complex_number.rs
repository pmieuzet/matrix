use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug, Copy)]
pub struct ComplexNumber<R: RealNumber> {
    pub x: R,
    pub y: R,
}

pub trait RealNumber:
    Add<f32, Output = f32>
    + std::cmp::PartialOrd<f32>
    + PartialEq<f32>
    + Mul<f32, Output = Self>
    + Mul<Output = Self>
    + Into<f32>
{
}
impl RealNumber for f32 {}
// impl<R: RealNumber> PartialOrd<f32> for ComplexNumber<R> {
//     fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
//         if self.x > other {
//             return Some(std::cmp::Ordering::Greater);
//         } else if self.x < other {
//             return Some(std::cmp::Ordering::Less);
//         }
//         Some(std::cmp::Ordering::Equal)
//     }
// }

impl<R: Add<Output = R> + RealNumber> Add<ComplexNumber<R>> for ComplexNumber<R> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<R: Sub<Output = R> + RealNumber> Sub<ComplexNumber<R>> for ComplexNumber<R> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<R> Mul<ComplexNumber<R>> for ComplexNumber<R>
where
    R: Add<Output = R> + Copy + RealNumber,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x + self.y * rhs.y,
            y: self.x * rhs.y + self.y * rhs.x,
        }
    }
}
impl<R> Mul<f32> for ComplexNumber<R>
where
    R: Add<Output = R> + Copy + RealNumber,
{
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl<R> Div<ComplexNumber<R>> for ComplexNumber<R>
where
    R: Div<Output = R> + Add<Output = R> + Sub<Output = R> + Copy + RealNumber,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: (self.x * rhs.x + self.y * rhs.y) / (rhs.x * rhs.x + rhs.x + rhs.y),
            y: (self.x * rhs.x - self.y * rhs.y) / (rhs.x * rhs.x + rhs.x + rhs.y),
        }
    }
}
