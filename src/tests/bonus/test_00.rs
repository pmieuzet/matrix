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
    fn test_00_add_03() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 2. },
            ComplexNumber { x: 2., y: 1. },
        ]);
        let v = Vector::from([ComplexNumber { x: 0., y: 1. }]);

        let result = u.add(v);
        let expected = Err(Error::NotSameSize);
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
            ComplexNumber { x: 1., y: -1. },
            ComplexNumber { x: 1., y: 1. },
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
        let u = Vector::from([ComplexNumber { x: 1., y: -2. }]);
        let result = u.mul(ComplexNumber { x: 1., y: 1. });
        let expected = Vector::from([ComplexNumber { x: -1., y: -1. }]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_02() {
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
    use crate::complex_number::ComplexNumber;
    use crate::errors::Error;
    use crate::matrix::Matrix;
    use std::ops::{Add, Mul, Sub};

    #[test]
    fn test_00_add_01() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 2., y: 2. },
            ],
            [
                ComplexNumber { x: 3., y: 1. },
                ComplexNumber { x: 4., y: 0. },
            ],
        ]);
        let v = Matrix::from([
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 1., y: 1. },
            ],
            [
                ComplexNumber { x: 2., y: 2. },
                ComplexNumber { x: -2., y: -2. },
            ],
        ]);
        let result = u.add(v);
        let expected = Ok(Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 3., y: 3. },
            ],
            [
                ComplexNumber { x: 5., y: 3. },
                ComplexNumber { x: 2., y: -2. },
            ],
        ]));
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_add_02() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 2., y: 2. },
            ],
            [
                ComplexNumber { x: 3., y: 1. },
                ComplexNumber { x: 4., y: 0. },
            ],
        ]);
        let v = Matrix::from([
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 1., y: 1. },
            ],
            [
                ComplexNumber { x: 2., y: 2. },
                ComplexNumber { x: -2., y: -2. },
            ],
            [
                ComplexNumber { x: 2., y: 2. },
                ComplexNumber { x: -2., y: -2. },
            ],
        ]);
        let result = u.add(v);
        let expected = Err(Error::NotSameSizeMatrix);
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_add_03() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 2., y: 2. },
            ],
            [
                ComplexNumber { x: 3., y: 1. },
                ComplexNumber { x: 4., y: 0. },
            ],
        ]);
        let v = Matrix::from([
            [ComplexNumber { x: 0., y: 0. }],
            [ComplexNumber { x: 2., y: 2. }],
        ]);
        let result = u.add(v);
        let expected = Err(Error::NotSameSizeMatrix);
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_sub_01() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 2., y: 2. },
            ],
            [
                ComplexNumber { x: 3., y: 1. },
                ComplexNumber { x: 4., y: 0. },
            ],
        ]);
        let v = Matrix::from([
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 1., y: 1. },
            ],
            [
                ComplexNumber { x: 2., y: 2. },
                ComplexNumber { x: -2., y: -2. },
            ],
        ]);
        let result = u.sub(v);
        let expected = Ok(Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 1., y: 1. },
            ],
            [
                ComplexNumber { x: 1., y: -1. },
                ComplexNumber { x: 6., y: 2. },
            ],
        ]));
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_sub_02() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 2., y: 2. },
            ],
            [
                ComplexNumber { x: 3., y: 1. },
                ComplexNumber { x: 4., y: 0. },
            ],
        ]);
        let v = Matrix::from([
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 1., y: 1. },
            ],
            [
                ComplexNumber { x: 2., y: 2. },
                ComplexNumber { x: -2., y: -2. },
            ],
            [
                ComplexNumber { x: 2., y: 2. },
                ComplexNumber { x: -2., y: -2. },
            ],
        ]);
        let result = u.sub(v);
        let expected = Err(Error::NotSameSizeMatrix);
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_sub_03() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 2., y: 2. },
            ],
            [
                ComplexNumber { x: 3., y: 1. },
                ComplexNumber { x: 4., y: 0. },
            ],
        ]);
        let v = Matrix::from([
            [ComplexNumber { x: 0., y: 0. }],
            [ComplexNumber { x: 2., y: 2. }],
        ]);
        let result = u.sub(v);
        let expected = Err(Error::NotSameSizeMatrix);
        assert_eq![result, expected];
    }

    #[test]
    fn test_00_mul_01() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 2., y: 2. },
            ],
            [
                ComplexNumber { x: 3., y: 1. },
                ComplexNumber { x: 4., y: 0. },
            ],
        ]);
        let result = u.mul(ComplexNumber { x: 2., y: 0. });
        let expected = Matrix::from([
            [
                ComplexNumber { x: 2., y: 2. },
                ComplexNumber { x: 4., y: 4. },
            ],
            [
                ComplexNumber { x: 6., y: 2. },
                ComplexNumber { x: 8., y: 0. },
            ],
        ]);
        assert_eq![result, expected];
    }
}
