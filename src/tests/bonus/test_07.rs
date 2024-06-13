#[cfg(test)]
mod test_07 {
    use std::ops::Mul;

    use crate::complex_number::ComplexNumber;
    use crate::errors::Error;
    use crate::{matrix::Matrix, vector::Vector};

    #[test]
    fn test_07_01() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 0., y: 0. },
            ],
            [
                ComplexNumber { x: 0., y: 1. },
                ComplexNumber { x: 1., y: 0. },
            ],
        ]);
        let v = Vector::from([
            ComplexNumber { x: 4., y: 2. },
            ComplexNumber { x: 2., y: 0. },
        ]);

        let result = u.mul(v);
        let expected = Ok(Vector::from([
            ComplexNumber { x: 6., y: 6. },
            ComplexNumber { x: 4., y: 4. },
        ]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_07_02() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 0., y: 0. },
            ],
            [
                ComplexNumber { x: 0., y: 1. },
                ComplexNumber { x: 1., y: 0. },
            ],
        ]);
        let v = Matrix::from([
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 1., y: 1. },
            ],
            [
                ComplexNumber { x: 1., y: 0. },
                ComplexNumber { x: 0., y: 1. },
            ],
        ]);
        let result = u.mul(v);
        let expected = Ok(Matrix::from([
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 2., y: 2. },
            ],
            [
                ComplexNumber { x: 1., y: 0. },
                ComplexNumber { x: 1., y: 2. },
            ],
        ]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_07_03() {
        let u = Matrix::from([[
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 0., y: 0. },
        ]]);
        let v = Vector::from([ComplexNumber { x: 4., y: 2. }]);

        let result = u.mul(v);
        let expected = Err(Error::WrongSizeMatrix);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_07_04() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 7., y: 8. },
            ],
            [
                ComplexNumber { x: 10., y: 1. },
                ComplexNumber { x: 1., y: 9. },
            ],
        ]);
        let v = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
        ]);

        let result = u.mul(v);
        let expected = Ok(Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
        ]));

        assert_eq!(result, expected);
    }
    #[test]
    fn test_07_05() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 0., y: 0. },
            ],
            [
                ComplexNumber { x: 0., y: 1. },
                ComplexNumber { x: 1., y: 0. },
            ],
        ]);
        let v = Matrix::from([
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 1., y: 1. },
            ],
            [
                ComplexNumber { x: 1., y: 0. },
                ComplexNumber { x: 0., y: 1. },
            ],
            [
                ComplexNumber { x: 1., y: 0. },
                ComplexNumber { x: 0., y: 1. },
            ],
        ]);

        let result = u.mul(v);
        let expected = Err(Error::WrongSizeMatrix);
        assert_eq!(result, expected);
    }
}
