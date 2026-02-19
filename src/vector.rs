use crate::{errors::Error, matrix, real_number};

use matrix::Matrix;
use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};
use real_number::RealNumber;

#[derive(Clone, Debug, PartialEq)]
pub struct Vector<K> {
    pub data: Vec<K>
}

/// To print a vector on the standard output
impl<K: std::fmt::Debug> Display for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K> {
    fn from(data: [K; N]) -> Self {
        Self::new(Vec::from(data))
    }
}

/// To reshape a matrix into a vector
impl<K> From<Matrix<K>> for Vector<K> {
    fn from(value: Matrix<K>) -> Self {
        let vector = value.data.into_iter().flatten().collect::<Vec<K>>();
        Self { data: vector }
    }
}

impl<K> Vector<K> {
    pub fn new(data: Vec<K>) -> Self {
        Self { data }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Return a scalar product of two vectors
    pub fn dot(&self, v: &Vector<K>) -> Result<K, Error>
    where
        K: Mul<Output = K> + Add<Output = K> + Copy + RealNumber,
    {
        if self.size() != v.size() {
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
        if self.size() != rhs.size() {
            return Err(Error::NotSameSize);
        }

        let data = self
            .data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<K>>();
        Ok(Self { data })
    }
}

/// Compute the subtraction of a vector by another vector
impl<K: Sub<Output = K>> Sub for Vector<K> {
    type Output = Result<Self, Error>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.size() != rhs.size() {
            return Err(Error::NotSameSize);
        }

        let data = self
            .data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a - b)
            .collect::<Vec<K>>();
        Ok(Self { data })
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
        Self { data }
    }
}
