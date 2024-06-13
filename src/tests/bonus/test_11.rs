#[cfg(test)]
mod test_11 {
    use crate::complex_number::ComplexNumber;
    use crate::errors::Error;
    use crate::matrix::Matrix;

    #[test]
    fn test_11_01() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 0. },
                ComplexNumber { x: -1., y: 0. },
            ],
            [
                ComplexNumber { x: -1., y: 1. },
                ComplexNumber { x: 1., y: 1. },
            ],
        ]);

        let result = u.determinant();
        let expected = Ok(ComplexNumber { x: 0., y: 2. });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_11_02() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 0. },
                ComplexNumber { x: -1., y: 0. },
                ComplexNumber { x: -1., y: 0. },
            ],
            [
                ComplexNumber { x: -1., y: 1. },
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: -1., y: 0. },
            ],
        ]);
        let result = u.determinant();
        let expected = Err(Error::NotSquareMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_11_03() {
        let u = Matrix::from([[
            ComplexNumber { x: 1., y: 0. },
            ComplexNumber { x: -1., y: 0. },
            ComplexNumber { x: -1., y: 0. },
        ]]);
        let result = u.determinant();
        let expected = Err(Error::NotSquareMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_11_04() {
        let u = Matrix::<ComplexNumber<f32>>::from([[]]);

        let result = u.determinant();
        let expected = Err(Error::NotSquareMatrix);

        assert_eq!(result, expected);
    }
}
