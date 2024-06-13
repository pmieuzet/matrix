#[cfg(test)]
mod test_09 {
    use crate::complex_number::ComplexNumber;
    use crate::errors::Error;
    use crate::matrix::Matrix;

    #[test]
    fn test_09_01() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 2. },
                ComplexNumber { x: 3., y: 4. },
                ComplexNumber { x: 5., y: 6. },
            ],
            [
                ComplexNumber { x: 5., y: 6. },
                ComplexNumber { x: 4., y: 3. },
                ComplexNumber { x: 1., y: 2. },
            ],
        ]);

        let result = u.transpose();
        let expected = Ok(Matrix::from([
            [
                ComplexNumber { x: 1., y: 2. },
                ComplexNumber { x: 5., y: 6. },
            ],
            [
                ComplexNumber { x: 3., y: 4. },
                ComplexNumber { x: 4., y: 3. },
            ],
            [
                ComplexNumber { x: 5., y: 6. },
                ComplexNumber { x: 1., y: 2. },
            ],
        ]));

        assert_eq!(result, expected);
    }
    #[test]
    fn test_09_02() {
        let u = Matrix::<ComplexNumber<f32>>::from([[]]);

        let result = u.transpose();
        let expected = Err(Error::EmptyMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_09_03() {
        let u = Matrix::<ComplexNumber<f32>>::from([[
            ComplexNumber { x: 5., y: 6. },
            ComplexNumber { x: 0., y: 1. },
        ]]);

        let result = u.transpose();
        let expected = Ok(Matrix::from([
            [ComplexNumber { x: 5., y: 6. }],
            [ComplexNumber { x: 0., y: 1. }],
        ]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_09_04() {
        let u = Matrix::<ComplexNumber<f32>>::from([[ComplexNumber { x: 0., y: 1. }]]);

        let result = u.transpose();
        let expected = Ok(Matrix::from([[ComplexNumber { x: 0., y: 1. }]]));
        assert_eq!(result, expected);
    }
}
