use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct ComplexNumber<R: RealNumber> {
    pub x: R,
    pub y: R,
}

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
impl<R> RealNumber for ComplexNumber<R>
where
    R: RealNumber + Mul<Output = R> + Add<Output = R> + Div<Output = R> + Sub<Output = R>,
{
    fn abs(self) -> f32 {
        (self.x.abs() * self.x.abs() + self.y.abs() * self.y.abs()).sqrt()
    }
    fn zero() -> Self {
        Self {
            x: R::zero(),
            y: R::zero(),
        }
    }
    fn one() -> Self {
        Self {
            x: R::one(),
            y: R::zero(),
        }
    }
    fn default() -> Self {
        Self {
            x: R::default(),
            y: R::default(),
        }
    }
}

impl<R: Add<Output = R> + RealNumber> Add<ComplexNumber<R>> for ComplexNumber<R> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<R: RealNumber + AddAssign> AddAssign<ComplexNumber<R>> for ComplexNumber<R> {
    fn add_assign(&mut self, rhs: ComplexNumber<R>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<R> Sub<ComplexNumber<R>> for ComplexNumber<R>
where
    R: RealNumber + Sub<Output = R>,
{
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
    R: RealNumber + Add<Output = R> + Mul<Output = R>,
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
    R: Mul<f32, Output = R> + RealNumber,
{
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl<R> SubAssign<ComplexNumber<R>> for ComplexNumber<R>
where
    R: RealNumber + Add<Output = R> + Mul<Output = R>,
{
    fn sub_assign(&mut self, rhs: ComplexNumber<R>) {
        self.x = self.x * rhs.x + self.y * rhs.y;
        self.y = self.x * rhs.y + self.y * rhs.x;
    }
}
// impl<R> Mul<f32> for ComplexNumber<R>
// where
//     R: RealNumber + Add<f32, Output = f32> + Mul<Output = R>,
//     f32: Add<R, Output = f32>,
// {
//     type Output = f32;
//     fn mul(self, rhs: f32) -> f32 {
//         self.x * self.x + rhs * rhs
//     }
// }
impl<R> Div<ComplexNumber<R>> for ComplexNumber<R>
where
    R: RealNumber + Add<Output = R> + Mul<Output = R> + Sub<Output = R> + Div<Output = R>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: (self.x * rhs.x + self.y * rhs.y) / (rhs.x * rhs.x + rhs.x + rhs.y),
            y: (self.x * rhs.x - self.y * rhs.y) / (rhs.x * rhs.x + rhs.x + rhs.y),
        }
    }
}

// impl<R> Div<f32> for ComplexNumber<R>
// where
//     R: Div<f32, Output = f32> + RealNumber + PartialEq<f32>,
// {
//     type Output = f32;
//     fn div(self, rhs: f32) -> Self::Output {
//         if self.y != 0.0 {
//             // eprintln!(
//             //     "You cannot divide a complex number with a float if the imaginary part is not null"
//             // );
//             return 0.0;
//         }
//         self.x / rhs
//     }
// }
impl<R> Div<f32> for ComplexNumber<R>
where
    R: Div<f32, Output = R> + RealNumber,
{
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
