#[cfg(test)]
mod test_10 {
    use crate::complex_number::ComplexNumber;
    use crate::matrix::Matrix;

    #[test]
    fn test_10_01() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 2., y: 1. },
                ComplexNumber { x: 2., y: 1. },
            ],
            [
                ComplexNumber { x: 1., y: 2. },
                ComplexNumber { x: 1., y: 2. },
            ],
        ]);
        let result = u.row_echelon();
        let expected = Matrix::from([
            [
                ComplexNumber {
                    x: 4.428569,
                    y: 4.0285683,
                },
                ComplexNumber {
                    x: 4.428569,
                    y: 4.0285683,
                },
            ],
            [
                ComplexNumber {
                    x: -3.3999972,
                    y: -2.9999976,
                },
                ComplexNumber {
                    x: -3.3999972,
                    y: -2.9999976,
                },
            ],
        ]);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_10_02() {
        let u = Matrix::from([[ComplexNumber { x: -9., y: 0. }]]);

        let result = u.row_echelon();
        let expected = Matrix::from([[ComplexNumber { x: 1.125, y: 1.125 }]]);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_10_03() {
        let u = Matrix::<ComplexNumber<f32>>::from([[]]);

        let result = u.row_echelon();
        let expected = Matrix::<ComplexNumber<f32>>::from([[]]);

        assert_eq!(result, expected);
    }
}
