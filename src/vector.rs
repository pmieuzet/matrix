use crate::matrix;

use matrix::Matrix;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Vector<K> {
    pub size: usize,
    pub data: Vec<K>,
}
impl<K> Display for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl<K: Clone, const N: usize> From<[K; N]> for Vector<K> {
    fn from(data: [K; N]) -> Self {
        Self::new(data.to_vec(), N)
    }
}
impl<K> Vector<K> {
    pub fn new(data: Vec<K>, size: usize) -> Self {
        Vector { size, data }
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn into_matrix(self) -> Matrix<K> {
        let matrix = self
            .data
            .into_iter()
            .map(|t| vec![t])
            .collect::<Vec<Vec<K>>>();

        Matrix {
            rows: matrix.len(),
            columns: 1,
            data: matrix,
        }
        // or
        // Matrix {
        //     rows: 1,
        //     colums: self.data.len(),
        //     data: vec![self.data],
        // }
    }
    pub fn add(&mut self, v: &Vector<K>) {}
    pub fn sub(&mut self, v: &Vector<K>) {}
    pub fn scl(&mut self, a: K) {}
}
