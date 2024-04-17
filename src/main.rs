use std::ops::{Add, Mul, Sub};

use matrix::Matrix;
use vector::Vector;

use crate::linear_combinaison::linear_combination;

mod errors;
mod linear_combinaison;
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
        println!("{}", u.add(v));
        // [7.0][10.0]
    }
    {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        println!("{}", u.sub(v));
        // [-3.0][-4.0]
    }
    {
        let u = Vector::from([2., 3.]);
        println!("{}", u.mul(2.));
        // [4.0][6.0]
    }
    {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        println!("{}", u.add(v));
        // [8.0, 6.0][1.0, 6.0]
    }
    {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        println!("{}", u.sub(v));
        // [-6.0, -2.0][5.0, 2.0]
    }
    {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        println!("{}", u.mul(2.));
        // [2.0, 4.0][6.0, 8.0]
    }

    println!("\n\n---------------EX01---------------\n");
    {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        match linear_combination(&[e1, e2, e3], &[10., -2., 0.5]) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("{e}"),
        }
        // [10.][-2.][0.5]
        match linear_combination(&[v1, v2], &[10., -2.]) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("{e}"),
        }
        // [10.][0.][230.]
    }
    println!("\n\n---------------EX02---------------\n");
}
