use crate::{
    complex_number::{ComplexNumber, RealNumber},
    errors::Error,
    matrix,
};

use matrix::Matrix;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Vector<K> {
    pub size: usize,
    pub data: Vec<K>,
}

/// To print a vector on the standart outpout
impl<K: std::fmt::Debug> Display for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K> {
    fn from(data: [K; N]) -> Self {
        Self::new(Vec::from(data), N)
    }
}

impl<K> Vector<K> {
    pub fn new(data: Vec<K>, size: usize) -> Self {
        Vector { size, data }
    }

    /// To return the size of a vector
    pub fn size(&self) -> usize {
        self.size
    }

    /// To reshape a vector into a matrix
    pub fn into_matrix(self) -> Matrix<K> {
        Matrix {
            rows: 1,
            columns: self.data.len(),
            data: vec![self.data],
        }
    }

    /// Return a scalar product of two vectors
    pub fn dot(&self, v: &Vector<K>) -> Result<K, Error>
    where
        K: Mul<Output = K> + Add<Output = K> + Copy + RealNumber,
    {
        if self.size != v.size {
            return Err(Error::NotSameSize);
        }

        self.data
            .iter()
            .zip(v.data.iter())
            .map(|(a, b)| *a * *b)
            .reduce(|acc, a| acc + a)
            .ok_or(Error::EmptyVector)
    }
}

impl<V: RealNumber> Vector<V> {
    /// ∥v∥1 : Manhattan norm / Taxicab norm
    pub fn norm_1(&self) -> f32 {
        self.data.iter().fold(0., |acc, a| a.abs() + acc)
    }
    /// ∥v∥ or ∥v∥2 : Euclidean norm
    pub fn norm(&self) -> f32 {
        self.data
            .iter()
            .fold(0., |acc, a| a.abs() * a.abs() + acc)
            .powf(0.5)
    }
    /// ∥v∥∞ : supremum norm / maximum norm
    pub fn norm_inf(&self) -> f32 {
        self.data.iter().fold(0., |acc, a| f32::max(acc, a.abs()))
    }
}

/// Compute the addition of two vectors
impl<K: Add<Output = K>> Add for Vector<K> {
    type Output = Result<Self, Error>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            return Err(Error::NotSameSize);
        }

        let data = self
            .data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<K>>();
        Ok(Self {
            size: data.len(),
            data,
        })
    }
}

/// Compute the subtraction of a vector by another vector
impl<K: Sub<Output = K>> Sub for Vector<K> {
    type Output = Result<Self, Error>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            return Err(Error::NotSameSize);
        }

        let data = self
            .data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a - b)
            .collect::<Vec<K>>();
        Ok(Self {
            size: data.len(),
            data,
        })
    }
}

/// Compute the scaling of a vector by a scalar
impl<K: Mul<Output = K> + Clone> Mul<K> for Vector<K> {
    type Output = Self;
    fn mul(self, scalar: K) -> Self::Output {
        let data = self
            .data
            .into_iter()
            .map(|x| x * scalar.clone())
            .collect::<Vec<K>>();
        Self {
            size: data.len(),
            data,
        }
    }
}

// dot (output = k)
// impl<K: Mul<Output = K> + Clone> Mul<Vector<K>> for Vector<K> {
//     type Output = K;
//     fn mul(self, scalar: Vector<K>) -> Self::Output {}
// }
