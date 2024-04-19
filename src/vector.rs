use crate::{errors::Error, matrix};

use matrix::Matrix;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
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

    /// Return a scalar product of two vectors
    pub fn dot(&self, v: Vector<K>) -> Result<K, Error>
    where
        K: Mul<Output = K> + Add<Output = K> + Clone,
    {
        if self.size != v.size {
            return Err(Error::NotSameSize);
        }

        self.to_owned()
            .data
            .into_iter()
            .zip(v.data.into_iter())
            .map(|(a, b)| a * b)
            .reduce(|acc, a| acc + a)
            .ok_or(Error::EmptyVector)
    }
}

impl<V> Vector<V>
where
    V: Add<f32, Output = f32>
        + std::cmp::PartialOrd<f32>
        + Mul<f32, Output = V>
        + Mul<Output = V>
        + Into<f32>
        + Clone,
{
    fn abs(x: &V) -> V {
        if *x < 0. {
            return x.clone() * -1.;
        }
        x.clone()
    }
    fn max(a: V, b: f32) -> f32 {
        if a > b {
            return a.into();
        }
        b
    }
    /// ∥v∥1 : Manhattan norm / Taxicab norm
    pub fn norm_1(&self) -> f32 {
        self.data.iter().fold(0., |acc, a| Self::abs(a) + acc)
    }
    /// ∥v∥ or ∥v∥2 : Euclidean norm
    pub fn norm(&self) -> f32 {
        self.data
            .iter()
            .fold(0., |acc, a| Self::abs(a) * Self::abs(a) + acc)
            .powf(0.5)
    }
    /// ∥v∥∞ : supremum norm / maximum norm
    pub fn norm_inf(&self) -> f32 {
        self.data
            .iter()
            .fold(0., |acc, a| Self::max(Self::abs(a), acc))
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

// dot (output = k)
// impl<K: Mul<Output = K> + Clone> Mul<Vector<K>> for Vector<K> {
//     type Output = K;
//     fn mul(self, scalar: Vector<K>) -> Self::Output {}
// }

impl<K: Div<Output = K> + Clone> Div<K> for Vector<K> {
    type Output = Self;
    fn div(self, scalar: K) -> Self::Output {
        let data = self
            .data
            .into_iter()
            .map(|x| x / scalar.clone())
            .collect::<Vec<K>>();
        Self {
            size: data.len(),
            data,
        }
    }
}
