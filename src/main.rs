use std::ops::{Add, Mul, Sub};

use complex_number::ComplexNumber;
use matrix::Matrix;
use vector::Vector;

use crate::functions::{angle_cos, cross_product, lerp, linear_combination};

mod complex_number;
mod errors;
mod functions;
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
    {
        match lerp(0., 1., 0.) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // 0.0
        match lerp(0., 1., 1.) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // 1.0
        match lerp(0., 1., 0.5) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // 0.5
        match lerp(21., 42., 0.3) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // 27.3
        match lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // [2.6][1.3]
        match lerp(
            Matrix::from([[2., 1.], [3., 4.]]),
            Matrix::from([[20., 10.], [30., 40.]]),
            0.5,
        ) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // [[11., 5.5][16.5, 22.]]
    }

    println!("\n\n---------------EX03---------------\n");
    {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([1., 1.]);
        match u.dot(&v) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // 0.0
    }
    {
        let u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        match u.dot(&v) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // 2.0
    }
    {
        let u = Vector::from([-1., 6.]);
        let v = Vector::from([3., 2.]);
        match u.dot(&v) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        } // 9.0
    }

    println!("\n\n---------------EX04---------------\n");
    {
        let u = Vector::from([0., 0., 0.]);
        println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
        // 0.0, 0.0, 0.0
    }
    {
        let u = Vector::from([1., 2., 3.]);
        println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
        // 6.0, 3.74165738, 3.0
    }
    {
        let u = Vector::from([-1., -2.]);
        println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
        // 3.0, 2.236067977, 2.0
    }

    println!("\n\n---------------EX05---------------\n");
    {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);
        match angle_cos(&u, &v) {
            Ok(cos) => println!("{}", cos),
            Err(e) => eprintln!("{e}"),
        }
        // 1.0
    }
    {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        match angle_cos(&u, &v) {
            Ok(cos) => println!("{}", cos),
            Err(e) => eprintln!("{e}"),
        } // 0.0
    }
    {
        let u = Vector::from([-1., 1.]);
        let v = Vector::from([1., -1.]);
        match angle_cos(&u, &v) {
            Ok(cos) => println!("{}", cos),
            Err(e) => eprintln!("{e}"),
        } // -1.0
    }
    {
        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        match angle_cos(&u, &v) {
            Ok(cos) => println!("{}", cos),
            Err(e) => eprintln!("{e}"),
        } // 1.0
    }
    {
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        match angle_cos(&u, &v) {
            Ok(cos) => println!("{}", cos),
            Err(e) => eprintln!("{e}"),
        } // 0.9746318
    }

    println!("\n\n---------------EX06---------------\n");
    {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0., 0.]);
        match cross_product(&u, &v) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // [0.][1.][0.]
    }
    {
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        match cross_product(&u, &v) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // [-3.][6.][-3.]
    }
    {
        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([-2., -5., 16.]);
        match cross_product(&u, &v) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // [17.][-58.][-16.]
    }

    println!("\n\n---------------EX07---------------\n");
    {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);
        match u.mul(v) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // [4.][2.]
    }
    {
        let u = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);
        match u.mul(v) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // [8.][4.]
    }
    {
        let u = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);
        match u.mul(v) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // [4.][-4.]
    }
    {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.], [0., 1.]]);
        match u.mul(v) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // [1., 0.][0., 1.]
    }
    {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        match u.mul(v) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // [2., 1.][4., 2.]
    }
    {
        let u = Matrix::from([[3., -5.], [6., 8.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        match u.mul(v) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{e}"),
        }
        // [-14., -7.][44., 22.]
    }

    println!("\n\n---------------EX08---------------\n");
    {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        match u.trace() {
            Ok(trace) => println!("{}", trace),
            Err(e) => eprintln!("{e}"),
        }
        // 2.0
    }
    {
        let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        match u.trace() {
            Ok(trace) => println!("{}", trace),
            Err(e) => eprintln!("{e}"),
        } // 9.0
    }
    {
        let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        match u.trace() {
            Ok(trace) => println!("{}", trace),
            Err(e) => eprintln!("{e}"),
        } // -21.0
    }

    println!("\n\n---------------EX09---------------\n");
    {
        let u = Matrix::from([[2., -9., 3.], [13., 11., -17.]]);
        println!("{}", u.transpose());
        // [[2., 13.][-9., 11][3., -17.]]
    }

    println!("\n\n---------------EX10---------------\n");
    {
        let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        println!("{}", u.row_echelon());
        // [1.0, 0.0, 0.0][0.0, 1.0, 0.0][0.0, 0.0, 1.0]
    }
    {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        println!("{}", u.row_echelon());
        // [1.0, 0.0][0.0, 1.0]
    }
    {
        let mut u = Matrix::from([[1., 2.], [2., 4.]]);
        println!("{}", u.row_echelon());
        // [1.0, 2.0][0.0, 0.0]
    }
    {
        let mut u = Matrix::from([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        println!("{}", u.row_echelon());
        // [1.0, 0.625, 0.0, 0.0, -12.1666667][0.0, 0.0, 1.0, 0.0, -3.6666667][0.0, 0.0, 0.0, 1.0, 29.5 ]
    }

    println!("\n\n---------------EX11---------------\n");
    {
        let u = Matrix::from([[1., -1.], [-1., 1.]]);
        match u.determinant() {
            Ok(det) => println!("{}", det),
            Err(e) => eprintln!("{e}"),
        }
        // 0.0
    }
    {
        let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        match u.determinant() {
            Ok(det) => println!("{}", det),
            Err(e) => eprintln!("{e}"),
        } // 8.0
    }
    {
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        match u.determinant() {
            Ok(det) => println!("{}", det),
            Err(e) => eprintln!("{e}"),
        } // -174.0
    }
    {
        let u = Matrix::from([
            [8., 5., -2., 4.],
            [4., 2.5, 20., 4.],
            [8., 5., 1., 4.],
            [28., -4., 17., 1.],
        ]);
        match u.determinant() {
            Ok(det) => println!("{}", det),
            Err(e) => eprintln!("{e}"),
        } // 1032
    }

    println!("\n\n---------------EX12---------------\n");
    {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        match u.inverse() {
            Ok(inverse) => println!("{}", inverse),
            Err(e) => eprintln!("{e}"),
        }
    }
    {
        // [1.0, 0.0, 0.0][0.0, 1.0, 0.0][0.0, 0.0, 1.0]
        let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        match u.inverse() {
            Ok(inverse) => println!("{}", inverse),
            Err(e) => eprintln!("{e}"),
        }
    }
    {
        // [0.5, 0.0, 0.0][0.0, 0.5, 0.0][0.0, 0.0, 0.5]
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        match u.inverse() {
            Ok(inverse) => println!("{}", inverse),
            Err(e) => eprintln!("{e}"),
        }
        // [0.649425287, 0.097701149, -0.655172414][-0.781609195, -0.126436782, 0.965517241][0.143678161, 0.074712644, -0.206896552]
    }

    println!("\n\n---------------EX13---------------\n");
    {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        println!("{}", u.rank());
        // 3
    }
    {
        let u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
        println!("{}", u.rank());
        // 2
    }
    {
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
        println!("{}", u.rank());
        // 3
    }

    println!("\n\n---------------EX00 -- BONUS---------------\n");
    {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 2. },
            ComplexNumber { x: 2., y: 1. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 4., y: 8. },
            ComplexNumber { x: 1., y: 2. },
        ]);
        println!("{}", u.add(v));
        // [5.0, 10.0][3.0, 3.0]
    }
    {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 2. },
            ComplexNumber { x: 2., y: 1. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 4., y: 8. },
            ComplexNumber { x: 1., y: 2. },
        ]);
        println!("{}", u.sub(v));
        // [-3.0, -6.0][1.0, -1.0]
    }
    {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 2. },
            ComplexNumber { x: 2., y: 1. },
        ]);
        println!("{}", u.mul(ComplexNumber { x: 2., y: 2. }));
        // [6.0, 6.0][6.0, 6.0]
    }
    {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 2. },
                ComplexNumber { x: 2., y: 1. },
            ],
            [
                ComplexNumber { x: 3., y: 4. },
                ComplexNumber { x: 4., y: 3. },
            ],
        ]);
        let v = Matrix::from([
            [
                ComplexNumber { x: 1., y: 2. },
                ComplexNumber { x: 2., y: 1. },
            ],
            [
                ComplexNumber { x: -1., y: 1. },
                ComplexNumber { x: -2., y: 2. },
            ],
        ]);
        println!("{}", u.add(v));
    }
    {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 2. },
                ComplexNumber { x: 2., y: 1. },
            ],
            [
                ComplexNumber { x: 3., y: 4. },
                ComplexNumber { x: 4., y: 3. },
            ],
        ]);
        let v = Matrix::from([
            [
                ComplexNumber { x: 1., y: 2. },
                ComplexNumber { x: 2., y: 1. },
            ],
            [
                ComplexNumber { x: -1., y: 1. },
                ComplexNumber { x: -2., y: 2. },
            ],
        ]);
        println!("{}", u.sub(v));
    }
    {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 2. },
                ComplexNumber { x: 2., y: 1. },
            ],
            [
                ComplexNumber { x: 3., y: 4. },
                ComplexNumber { x: 4., y: 3. },
            ],
        ]);
        println!("{}", u.mul(ComplexNumber { x: 1., y: 2. }));
    }

    println!("\n\n---------------EX01 -- BONUS---------------\n");
    {
        let e1 = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
        ]);
        let e2 = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 0., y: 0. },
        ]);
        let e3 = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 1., y: 1. },
        ]);
        match linear_combination(
            &[e1, e2, e3],
            &[
                ComplexNumber { x: 10., y: 2. },
                ComplexNumber { x: -2., y: 0. },
                ComplexNumber { x: 0.5, y: 0.5 },
            ],
        ) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("{e}"),
        }
    }
    println!("\n\n---------------EX02 -- BONUS---------------\n");
    {
        match lerp(
            ComplexNumber { x: 10., y: 0. },
            ComplexNumber { x: 1., y: 2. },
            0.,
        ) {
            Ok(res) => println!("{:?}", res),
            Err(e) => eprintln!("{e}"),
        }
        match lerp(
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 1., y: 1. },
            1.,
        ) {
            Ok(res) => println!("{:?}", res),
            Err(e) => eprintln!("{e}"),
        }
    }
    println!("\n\n---------------EX03 -- BONUS---------------\n");
    {
        let u = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 1., y: 1. },
        ]);
        match u.dot(&v) {
            Ok(res) => println!("{:?}", res),
            Err(e) => eprintln!("{e}"),
        }
        // 0.0
    }
    {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 1., y: 1. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 1., y: 1. },
        ]);
        match u.dot(&v) {
            Ok(res) => println!("{:?}", res),
            Err(e) => eprintln!("{e}"),
        }
    }

    println!("\n\n---------------EX04 -- BONUS---------------\n");
    // {
    //     let u = Vector::from([
    //         ComplexNumber { x: 0., y: 0. },
    //         ComplexNumber { x: 0., y: 0. },
    //         ComplexNumber { x: 0., y: 0. },
    //     ]);
    //     println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    //     // 0.0, 0.0, 0.0
    // }
    // {
    //     let u = Vector::from([1., 2., 3.]);
    //     println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    //     // 6.0, 3.74165738, 3.0
    // }
    // {
    //     let u = Vector::from([-1., -2.]);
    //     println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    //     // 3.0, 2.236067977, 2.0
    // }
}
