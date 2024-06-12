#[cfg(test)]
mod test_vector_00 {
    use crate::complex_number::ComplexNumber;
    use crate::errors::Error;
    use crate::vector::Vector;
    use std::ops::{Add, Mul, Sub};

    #[test]
    fn test_00_add_01() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 2. },
            ComplexNumber { x: 2., y: 1. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 4., y: 8. },
            ComplexNumber { x: 1., y: 2. },
        ]);

        let result = u.add(v);
        let expected = Ok(Vector::from([
            ComplexNumber { x: 5., y: 10. },
            ComplexNumber { x: 3., y: 3. },
        ]));
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_add_02() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 2. },
            ComplexNumber { x: 2., y: 1. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 0., y: 1. },
            ComplexNumber { x: 1., y: 0. },
        ]);

        let result = u.add(v);
        let expected = Ok(Vector::from([
            ComplexNumber { x: 1., y: 3. },
            ComplexNumber { x: 3., y: 1. },
        ]));
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_sub_01() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 2. },
            ComplexNumber { x: 2., y: 1. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 4., y: 8. },
            ComplexNumber { x: 1., y: 2. },
        ]);

        let result = u.sub(v);
        let expected = Ok(Vector::from([
            ComplexNumber { x: -3., y: -6. },
            ComplexNumber { x: 1., y: -1. },
        ]));
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_sub_02() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: -2. },
            ComplexNumber { x: 2., y: 1. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 0., y: -1. },
            ComplexNumber { x: 1., y: 0. },
        ]);

        let result = u.sub(v);
        let expected = Ok(Vector::from([
            ComplexNumber { x: 0., y: 2. },
            ComplexNumber { x: 2., y: 0. },
        ]));
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_sub_03() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: -2. },
            ComplexNumber { x: 2., y: 1. },
        ]);
        let v = Vector::from([ComplexNumber { x: 0., y: -1. }]);

        let result = u.sub(v);
        let expected = Err(Error::NotSameSize);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_01() {
        let u = Vector::from([2., 3.]);
        let result = u.mul(2.);
        let expected = Vector::from([4., 6.]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_03() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 2. },
            ComplexNumber { x: 2., y: 1. },
        ]);

        let result = u.mul(ComplexNumber { x: 2., y: 2. });
        let expected = Vector::from([
            ComplexNumber { x: 6., y: 6. },
            ComplexNumber { x: 6., y: 6. },
        ]);
        assert_eq![result, expected];
    }
}

#[cfg(test)]
mod test_matrix_01 {
    use crate::matrix::Matrix;
    use std::ops::{Add, Mul, Sub};

    #[test]
    fn test_00_add_01() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        let result = u.add(v);
        let expected = Ok(Matrix::from([[8., 6.], [1., 6.]]));
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_add_02() {
        let u = Matrix::from([[-1., -2.], [-3., 4.]]);
        let v = Matrix::from([[0., -2.], [2., -2.]]);
        let result = u.add(v);
        let expected = Ok(Matrix::from([[-1., -4.], [-1., 2.]]));
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_add_03() {
        let u = Matrix::from([[1.1, 2.0], [3.2, 4.2]]);
        let v = Matrix::from([[7.7, 4.4], [-2.2, 2.2]]);
        let result = u.add(v);
        let expected = Ok(Matrix::from([[8.8, 6.4], [1.0, 6.4]]));
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_sub_01() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        let result = u.sub(v);
        let expected = Ok(Matrix::from([[-6.0, -2.0], [5.0, 2.0]]));
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_sub_02() {
        let u = Matrix::from([[1., -2.], [3., -4.]]);
        let v = Matrix::from([[-1., -2.], [-2., 2.]]);
        let result = u.sub(v);
        let expected = Ok(Matrix::from([[2.0, 0.0], [5.0, -6.0]]));
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_sub_03() {
        let u = Matrix::from([[1.1, 2.4], [3.2, 4.2]]);
        let v = Matrix::from([[7.7, 4.0], [-2.2, 2.2]]);
        let result = u.sub(v);
        let expected = Ok(Matrix::from([[-6.6, -1.6], [5.4, 2.0]]));
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_mul_01() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let result = u.mul(2.);
        let expected = Matrix::from([[2.0, 4.0], [6.0, 8.0]]);
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_mul_02() {
        let u = Matrix::from([[-1., 0.], [-3., -2.]]);
        let result = u.mul(2.);
        let expected = Matrix::from([[-2., 0.], [-6., -4.]]);
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_mul_03() {
        let u = Matrix::from([[-1., 0.], [-3., 2.]]);
        let result = u.mul(-2.);
        let expected = Matrix::from([[2., 0.], [6., -4.]]);
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_mul_04() {
        let u = Matrix::from([[-1., 0.], [-3., 2.]]);
        let result = u.mul(0.);
        let expected = Matrix::from([[0., 0.], [0., 0.]]);
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_sub_05() {
        let u = Matrix::from([[1.0, 2.4], [3.0, 4.01]]);
        let result = u.mul(2.22);
        let expected = Matrix::from([[2.22, 5.328], [6.66, 8.9022]]);
        assert_eq![result, expected];
    }
}
