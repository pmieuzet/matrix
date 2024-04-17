use crate::matrix;

use matrix::Matrix;
use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

#[derive(Clone, Debug)]
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
}

/// Compute the addition of two vectors
impl<K: Add<Output = K>> Add for Vector<K> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let data = self
            .data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<K>>();
        Self {
            size: data.len(),
            data,
        }
    }
}

/// Compute the subtraction of a vector by another vector
impl<K: Sub<Output = K>> Sub for Vector<K> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let data = self
            .data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a - b)
            .collect::<Vec<K>>();
        Self {
            size: data.len(),
            data,
        }
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
