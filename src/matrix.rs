use crate::vector;

use std::fmt::Display;
use vector::Vector;

#[derive(Clone, Debug)]
pub struct Matrix<K> {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<Vec<K>>,
}
impl<K> Display for Matrix<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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
    pub fn size(&self) -> usize {
        self.columns * self.rows
    }
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }
    pub fn is_square(&self) -> bool {
        self.rows == self.columns
    }
    pub fn into_vector(self) -> Vector<K> {
        let vector = self.data.into_iter().flatten().collect::<Vec<K>>();
        Vector {
            size: vector.len(),
            data: vector,
        }
    }
    pub fn add(&mut self, v: &Matrix<K>) {}
    pub fn sub(&mut self, v: &Matrix<K>) {}
    pub fn scl(&mut self, a: K) {}
}
