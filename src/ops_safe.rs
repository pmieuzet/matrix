use crate::complex_number::{ComplexNumber, RealNumber};
use crate::errors::Error;
use crate::matrix::Matrix;
use crate::vector::Vector;
use std::ops::{Add, Div, Mul, Sub};

pub trait DivSafe {
    fn div(self, rhs: f32) -> Result<f32, Error>;
}
impl DivSafe for f32 {
    fn div(self, rhs: f32) -> Result<f32, Error> {
        if rhs == 0.0 {
            return Err(Error::DivisionByZero);
        }
        Ok(self / rhs)
    }
}
impl<R> DivSafe for ComplexNumber<R>
where
    R: RealNumber + PartialEq<f32> + Div<f32, Output = f32>,
{
    fn div(self, rhs: f32) -> Result<f32, Error> {
        if self.y != 0.0 {
            return Err(Error::NotImaginaryPartOfComplexNumber);
        } else if rhs == 0.0 {
            return Err(Error::DivisionByZero);
        }
        Ok(self.x / rhs)
    }
}

pub trait SubSafe
where
    Self: Sized,
{
    fn sub(self, rhs: Self) -> Result<Self, Error>;
}
impl SubSafe for f32 {
    fn sub(self, rhs: Self) -> Result<Self, Error> {
        Ok(self - rhs)
    }
}
impl<R> SubSafe for ComplexNumber<R>
where
    R: RealNumber + Sub<Output = R>,
{
    fn sub(self, rhs: Self) -> Result<Self, Error> {
        Ok(self - rhs)
    }
}
impl<R> SubSafe for Vector<R>
where
    R: Sub<Output = R> + RealNumber,
{
    fn sub(self, rhs: Self) -> Result<Self, Error> {
        self - rhs
    }
}
impl<R> SubSafe for Matrix<R>
where
    R: Sub<Output = R> + RealNumber,
{
    fn sub(self, rhs: Self) -> Result<Self, Error> {
        self - rhs
    }
}

pub trait AddSafe
where
    Self: Sized,
{
    fn add(self, rhs: Self) -> Result<Self, Error>;
}
impl AddSafe for f32 {
    fn add(self, rhs: Self) -> Result<Self, Error> {
        Ok(self + rhs)
    }
}
impl<R> AddSafe for ComplexNumber<R>
where
    R: RealNumber + Add<Output = R>,
{
    fn add(self, rhs: Self) -> Result<Self, Error> {
        Ok(self + rhs)
    }
}
impl<R> AddSafe for Vector<R>
where
    R: Add<Output = R> + RealNumber,
{
    fn add(self, rhs: Self) -> Result<Self, Error> {
        self + rhs
    }
}
impl<R> AddSafe for Matrix<R>
where
    R: Add<Output = R> + RealNumber,
{
    fn add(self, rhs: Self) -> Result<Self, Error> {
        self + rhs
    }
}

pub trait MulByf32 {
    fn mul(self, rhs: f32) -> Self;
}
impl MulByf32 for f32 {
    fn mul(self, rhs: f32) -> Self {
        self * rhs
    }
}
impl<R> MulByf32 for ComplexNumber<R>
where
    R: Mul<f32, Output = R> + RealNumber,
{
    fn mul(self, rhs: f32) -> Self {
        self * rhs
    }
}
impl<R> MulByf32 for Vector<R>
where
    R: Mul<f32, Output = R> + Clone,
{
    fn mul(self, rhs: f32) -> Self {
        let data = self.data.iter().map(|x| x.clone() * rhs).collect();
        Self { data }
    }
}
impl<R> MulByf32 for Matrix<R>
where
    R: Mul<f32, Output = R> + Copy,
{
    fn mul(self, rhs: f32) -> Self {
        let data = self
            .data
            .iter()
            .map(|x| x.iter().map(|y| *y * rhs.clone()).collect())
            .collect();
        Self {
            columns: self.columns,
            rows: self.rows,
            data,
        }
    }
}
