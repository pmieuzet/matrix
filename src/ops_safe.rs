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
impl SubSafe for Matrix<f32> {
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
impl AddSafe for Matrix<f32> {
    fn add(self, rhs: Self) -> Result<Self, Error> {
        self + rhs
    }
}
