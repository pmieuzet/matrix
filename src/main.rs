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
    println!("Matrix: {:?}", matrix);

    let vector = Vector {
        size: 2,
        data: vec![5., 6.],
    };
    println!("Vector: {:?}", vector);

    println!("\n\n---------------EX00---------------\n");
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    // u.add(v);
    println!("{}", u);
    // [7.0]
    // [10.0]
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    // u.sub(v);
    println!("{}", u);
    // [-3.0]
    // [-4.0]
    let mut u = Vector::from([2., 3.]);
    // u.scl(2.);
    println!("{}", u);
    // [4.0]
    // [6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    // u.add(v);
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    //u.add(v);
    println!("{}", u);
    // [7.0]
    // [10.0]
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    // u.sub(v);
    println!("{}", u);
    // [-3.0]
    // [-4.0]
    let mut u = Vector::from([2., 3.]);
    // u.scl(2.);
    println!("{}", u);
    // [4.0]
    // [6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    // u.add(v);
}
