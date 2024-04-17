use std::ops::{Add, Mul, Sub};

use matrix::Matrix;

use crate::vector::Vector;

mod matrix;
mod vector;

fn main() {
    let matrix = Matrix {
        columns: 2,
        rows: 2,
        data: vec![vec![1., 2.], vec![2., 3.]],
    };
    println!("Matrix: {}", matrix);

    let vector = Vector {
        size: 2,
        data: vec![5., 6.],
    };
    println!("Vector: {}", vector);

    println!("\n\n---------------EX00---------------\n");
    {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        println!("{}", u.clone().add(v));
        // [7.0]
        // [10.0]
    }
    {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.addition(&v);
        println!("{}", u);
        // [7.0]
        // [10.0]
    }
    {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        println!("{}", u.clone().sub(v));
        // [-3.0]
        // [-4.0]
    }
    {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.subtraction(&v);
        println!("{}", u);
        // [-3.0]
        // [-4.0]
    }
    {
        let mut u = Vector::from([2., 3.]);
        u.scl(2.);
        println!("{}", u);
        // [4.0]
        // [6.0]
    }
    {
        let u = Vector::from([2., 3.]);
        println!("{}", u.mul(2.));
        // [4.0]
        // [6.0]
    }
    {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u.addition(&v);
        println!("{}", u);
        // [8.0, 6.0]
        // [1.0, 6.0]
    }
    {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        println!("{}", u.add(v));
        // [8.0, 6.0]
        // [1.0, 6.0]
    }
    {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u.subtraction(&v);
        println!("{}", u);
        // [-6.0, -2.0]
        // [5.0, 2.0]
    }
    {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        println!("{}", u.sub(v));
        // [-6.0, -2.0]
        // [5.0, 2.0]
    }
    {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        u.scl(2.);
        println!("{}", u);
        // [2.0, 4.0]
        // [6.0, 8.0]
    }
    {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        println!("{}", u.mul(2.));
        // [2.0, 4.0]
        // [6.0, 8.0]
    }
}
