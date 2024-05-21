use crate::{errors::Error, vector};
use std::fmt::Debug;

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign},
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
    pub fn transpose(&self) -> Self
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

        Self {
            rows: self.columns,
            columns: self.rows,
            data,
        }
    }

    // TODO /// Compute the row-echelon form of the given matrix
    // pub fn row_echelon(&self) -> Self {}

    fn get_sub_matrix(&self, n: usize) -> Self
    where
        K: Copy,
    {
        let mut data = vec![];
        for i in 1..self.rows {
            let mut vector = vec![];
            for j in 0..self.columns {
                if j != n {
                    vector.push(self.data[i][j]);
                }
            }
            data.push(vector);
        }
        Matrix {
            rows: self.rows - 1,
            columns: self.columns - 1,
            data,
        }
    }

    /// Compute the determinant of the given matrix
    pub fn determinant(&self) -> Result<K, Error>
    where
        K: Mul<Output = K> + Sub<Output = K> + Default + Copy + AddAssign + SubAssign,
    {
        if !self.is_square() || self.rows < 2 {
            return Err(Error::NotSquareMatrix);
        }

        if self.rows == 2 {
            return Ok(self.data[0][0] * self.data[1][1] - self.data[1][0] * self.data[0][1]);
        }

        let mut det = K::default();
        let mut add = true;
        for n in 0..self.columns {
            if add == true {
                det += self.data[0][n] * self.get_sub_matrix(n).determinant().unwrap();
            } else {
                det -= self.data[0][n] * self.get_sub_matrix(n).determinant().unwrap();
            }
            add = !add;
        }
        Ok(det)
    }

    /// Compute the row echelon
    pub fn row_echelon(&mut self) -> Matrix<K>
    where
        K: Div<f32, Output = K>
            + MulAssign
            + Copy
            + PartialOrd<f32>
            + Div<f32, Output = K>
            + Sub<Output = K>
            + Mul<Output = K>
            + Debug,
        f32: Div<K, Output = K>,
    {
        let mut data = self.data.clone();
        let mut mark = 0;
        for m in 0..self.rows {
            for n in mark..self.columns {
                for i in m..self.rows {
                    if i == m && data[i][n] > 1. {
                        println!("1 -> i : {i}");
                        data[i] = data[i].iter().map(|a| *a * (1. / data[i][n])).collect();
                    } else if i == m && data[i][n] <= 1. {
                        println!("2 -> i : {i}");
                        mark += i;
                        continue;
                    } else if i != m && data[i][n] != 0. {
                        println!("3 -> i : {i}, n : {n} => {:?}", data[i][n]);
                        data[i] = data[i]
                            .iter()
                            .map(|a| *a - data[i][n] * (data[m][n] / 1.))
                            .collect();
                    } else if i != m && data[i][n] == 0. {
                        println!("4 -> i : {i}");
                        data[i] = data[i].iter().map(|a| *a * data[m][n]).collect();
                    } else {
                        println!("5 -> i : {i}");
                    }
                }
                if data[m][n] == 1. {
                    mark += 1;
                    break;
                }
            }
        }

        // for mut m in 0..self.rows {
        //     for n in m..self.rows {
        //         for i in 0..self.columns {
        //             if m == n {
        //                 if i == mark && data[m][i] > 1. {
        //                     data[i] = data[i].iter().map(|a| *a * (1. / data[m][i])).collect();
        //                     println!("1");
        //                 }
        //                 if i == mark && data[m][i] == 1. {
        //                     println!("1bis");
        //                     mark += 1;
        //                     m += 1;
        //                     break;
        //                 } else if data[m][i] == 0. {
        //                     println!("2");
        //                     continue;
        //                 } else {
        //                     data[n] = data[n].iter().map(|a| *a * data[m][i]).collect();
        //                     println!("3");

        //                     continue;
        //                 }
        //             }
        //             if data[n][i] == 0. {
        //                 data[n] = data[n].iter().map(|a| *a * data[m][i]).collect();
        //                 println!("3bis");
        //             } else {
        //                 data[n] = data[n]
        //                     .iter()
        //                     .map(|a| *a - data[n][i] * (data[m][i] / 1.))
        //                     .collect();
        //             }
        //             println!("4");
        //         }
        //     }
        // }

        Matrix {
            rows: self.rows,
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
