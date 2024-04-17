use crate::vector;

use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};
use vector::Vector;

#[derive(Clone, Debug)]
pub struct Matrix<K> {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<Vec<K>>,
}

//To print a matrix on the standart input
impl<K: std::fmt::Debug> Display for Matrix<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<K: Clone, const N: usize, const U: usize> From<[[K; N]; U]> for Matrix<K> {
    fn from(data: [[K; N]; U]) -> Self {
        let mut matrice = Vec::new();
        for item in data {
            matrice.push(item.to_vec());
        }
        Self::new(matrice, N, U)
    }
}

impl<K> Matrix<K> {
    pub fn new(data: Vec<Vec<K>>, rows: usize, columns: usize) -> Matrix<K> {
        Matrix {
            rows,
            columns,
            data,
        }
    }

    //To return the size of a matrix
    pub fn size(&self) -> usize {
        self.columns * self.rows
    }

    //To return the shape of a matrix
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }

    //To tell if a matrix is square
    pub fn is_square(&self) -> bool {
        self.rows == self.columns
    }

    //To reshape a matrix into a vector
    pub fn into_vector(self) -> Vector<K> {
        let vector = self.data.into_iter().flatten().collect::<Vec<K>>();
        Vector {
            size: vector.len(),
            data: vector,
        }
    }

    //Compute the addition of two matrix
    pub fn addition(&mut self, v: &Matrix<K>)
    where
        K: std::ops::AddAssign,
        K: Clone,
    {
        for i in 0..self.rows {
            for j in 0..self.columns {
                self.data[i][j] += v.data[i][j].clone();
            }
        }
    }

    //Compute the subtraction of a matrix by another matrix
    pub fn subtraction(&mut self, v: &Matrix<K>)
    where
        K: std::ops::SubAssign,
        K: Clone,
    {
        for i in 0..self.rows {
            for j in 0..self.columns {
                self.data[i][j] -= v.data[i][j].clone();
            }
        }
    }

    //Compute the scaling of a vector by a scalar
    pub fn scl(&mut self, a: K)
    where
        K: std::ops::MulAssign,
        K: Clone,
    {
        for item in &mut self.data {
            for x in item.iter_mut() {
                *x *= a.clone();
            }
        }
    }
}

impl<K: Add<Output = K> + Clone> Add for Matrix<K> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut data = vec![];
        for i in 0..self.rows {
            let mut vector = vec![];
            for j in 0..self.columns {
                vector.push(self.data[i][j].clone() + rhs.data[i][j].clone())
            }
            data.push(vector);
        }
        Self {
            rows: self.rows,
            columns: self.columns,
            data,
        }
    }
}

impl<K: Sub<Output = K> + Clone> Sub for Matrix<K> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = vec![];
        for i in 0..self.rows {
            let mut vector = vec![];
            for j in 0..self.columns {
                vector.push(self.data[i][j].clone() - rhs.data[i][j].clone())
            }
            data.push(vector);
        }
        Self {
            rows: self.rows,
            columns: self.columns,
            data,
        }
    }
}

impl<K: Mul<Output = K> + Clone> Mul<K> for Matrix<K> {
    type Output = Self;
    fn mul(self, scalar: K) -> Self::Output {
        let mut data = vec![];
        for item in self.data {
            let mut vector = vec![];
            for x in item.into_iter() {
                vector.push(x * scalar.clone());
            }
            data.push(vector);
        }
        Self {
            rows: self.rows,
            columns: self.columns,
            data,
        }
    }
}
