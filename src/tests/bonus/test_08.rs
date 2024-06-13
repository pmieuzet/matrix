#[cfg(test)]
mod test_08 {
    use crate::complex_number::ComplexNumber;
    use crate::errors::Error;
    use crate::matrix::Matrix;

    #[test]
    fn test_08_01() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 1. },
                ComplexNumber { x: 0., y: 1. },
            ],
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 1., y: 0. },
            ],
        ]);

        let result = u.trace();
        let expected = Ok(ComplexNumber { x: 2., y: 1. });

        assert_eq!(result, expected);
    }
    #[test]
    fn test_08_02() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 6., y: 1. },
                ComplexNumber { x: 0., y: 1. },
                ComplexNumber { x: 12., y: 0. },
            ],
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 8., y: 12. },
                ComplexNumber { x: 1., y: 0. },
            ],
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 9., y: 9. },
                ComplexNumber { x: 6., y: 0. },
            ],
        ]);

        let result = u.trace();
        let expected = Ok(ComplexNumber { x: 20., y: 13. });

        assert_eq!(result, expected);
    }
    #[test]
    fn test_08_03() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 6., y: 1. },
                ComplexNumber { x: 0., y: 1. },
            ],
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 8., y: 12. },
            ],
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 9., y: 9. },
            ],
        ]);

        let result = u.trace();
        let expected = Err(Error::NotSquareMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_08_04() {
        let u = Matrix::<ComplexNumber<f32>>::from([[]]);

        let result = u.trace();
        let expected = Err(Error::EmptyMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_08_05() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 0., y: 1. },
                ComplexNumber { x: 9., y: 9. },
            ],
            [
                ComplexNumber { x: 8., y: 12. },
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 9., y: 9. },
            ],
            [
                ComplexNumber { x: 0., y: 0. },
                ComplexNumber { x: 9., y: 9. },
                ComplexNumber { x: 0., y: 0. },
            ],
        ]);
        let result = u.trace();
        let expected = Ok(ComplexNumber { x: 0., y: 0. });

        assert_eq!(result, expected);
    }
}
