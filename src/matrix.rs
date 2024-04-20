use crate::{errors::Error, vector};

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Sub},
    process::Output,
    vec,
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

impl<K, const N: usize, const U: usize> From<[[K; N]; U]> for Matrix<K> {
    fn from(data: [[K; N]; U]) -> Self {
        let mut matrice = Vec::new();
        for item in data {
            matrice.push(Vec::from(item));
        }
        Self::new(matrice, U, N)
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

    ///  Compute the trace of the given matrix
    pub fn trace(&self) -> Result<K, Error>
    where
        K: AddAssign + Copy,
    {
        if !self.is_square() {
            return Err(Error::NotSquareMatrix);
        }

        let mut trace = self.data[0][0];
        for n in 1..self.rows {
            trace += self.data[n][n];
        }

        Ok(trace)
    }

    /// Compute the transpose matrix of a given matrix
    pub fn transpose(&self) -> Matrix<K>
    where
        K: Copy,
    {
        let mut data = vec![];
        for n in 0..self.columns {
            let mut vector = vec![];
            for m in 0..self.rows {
                vector.push(self.data[m][n]);
            }
            data.push(vector);
        }

        Matrix {
            rows: self.columns,
            columns: self.rows,
            data,
        }
    }
}

/// Compute the addition of two matrix
impl<K: Add<Output = K> + Clone> Add for Matrix<K> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut data = vec![];
        for (vec1, vec2) in self.data.into_iter().zip(rhs.data.into_iter()) {
            let mut vector = vec![];
            for (a, b) in vec1.into_iter().zip(vec2.into_iter()) {
                vector.push(a + b);
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

/// Compute the subtraction of a matrix by another matrix
impl<K: Sub<Output = K> + Clone> Sub for Matrix<K> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = vec![];
        for (vec1, vec2) in self.data.into_iter().zip(rhs.data.into_iter()) {
            let mut vector = vec![];
            for (a, b) in vec1.into_iter().zip(vec2.into_iter()) {
                vector.push(a - b);
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

/// Compute the scaling of a vector by a scalar
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

/// Multiply a matrix by a vector
impl<K: Mul<Output = K> + AddAssign + Copy> Mul<Vector<K>> for Matrix<K> {
    type Output = Result<Vector<K>, Error>;
    fn mul(self, rhs: Vector<K>) -> Self::Output {
        if self.columns != rhs.size {
            return Err(Error::WrongSizeMatrix);
        }

        let mut data = vec![];
        for m in 0..self.rows {
            let mut acc = self.data[m][0] * rhs.data[0];
            for n in 1..rhs.size {
                acc += self.data[m][n] * rhs.data[n];
            }
            data.push(acc);
        }

        Ok(Vector {
            size: data.len(),
            data,
        })
    }
}

/// Multiply a matrix by a matrix
impl<K: Mul<Output = K> + Copy + AddAssign> Mul<Matrix<K>> for Matrix<K> {
    type Output = Result<Self, Error>;
    fn mul(self, rhs: Matrix<K>) -> Self::Output {
        if self.columns != rhs.rows {
            return Err(Error::WrongSizeMatrix);
        }

        let mut data = vec![];
        for m in 0..self.rows {
            let mut vector = vec![];
            for p in 0..rhs.columns {
                let mut acc = self.data[m][0] * rhs.data[0][p];
                for n in 1..self.columns {
                    acc += self.data[m][n] * rhs.data[n][p];
                }
                vector.push(acc);
            }
            data.push(vector);
        }

        Ok(Matrix {
            rows: self.rows,
            columns: rhs.columns,
            data,
        })
    }
}
