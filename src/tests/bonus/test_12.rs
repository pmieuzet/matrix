#[cfg(test)]
mod test_12 {
    use crate::complex_number::ComplexNumber;
    use crate::errors::Error;
    use crate::matrix::Matrix;

    #[test]
    fn test_12_01() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 0. },
                ComplexNumber { x: 0., y: 0. },
            ],
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 1., y: 0. },
            ],
        ]);
        let result = u.inverse();
        let expected = Ok(Matrix::from([
            [
                ComplexNumber { x: 0.5, y: 0.25 },
                ComplexNumber { x: 0., y: 0. },
            ],
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 0.5, y: 0.25 },
            ],
        ]));

        assert_eq!(result, expected);
    }
    #[test]
    fn test_12_04() {
        let u = Matrix::from([
            [ComplexNumber { x: 1., y: 0. }],
            [ComplexNumber { x: 0., y: 0. }],
        ]);
        let result = u.inverse();
        let expected = Err(Error::NotSquareMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_12_05() {
        let u = Matrix::from([[
            ComplexNumber { x: 1., y: 0. },
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 1., y: 0. },
        ]]);
        let result = u.inverse();
        let expected = Err(Error::NotSquareMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_12_06() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 0. },
                ComplexNumber { x: -1., y: 0. },
            ],
            [
                ComplexNumber { x: -1., y: 0. },
                ComplexNumber { x: 1., y: 0. },
            ],
        ]);
        let result = u.inverse();
        let expected = Err(Error::NullDeterminantMatrix);

        assert_eq!(result, expected);
    }
}
