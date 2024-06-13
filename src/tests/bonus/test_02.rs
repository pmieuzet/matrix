#[cfg(test)]
mod test_02 {
    use crate::complex_number::ComplexNumber;
    use crate::errors::Error;
    use crate::functions::lerp;
    use crate::matrix::Matrix;
    use crate::vector::Vector;

    #[test]
    fn test_02_01() {
        let result = lerp(
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 1., y: 0. },
            0.,
        );
        let expected = Ok(ComplexNumber { x: 0., y: 0. });
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_02() {
        let result = lerp(
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 1., y: 0. },
            1.,
        );
        let expected = Ok(ComplexNumber { x: 1., y: 0. });
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_03() {
        let result = lerp(
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 1., y: 0. },
            0.5,
        );
        let expected = Ok(ComplexNumber { x: 0.5, y: 0. });
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_04() {
        let result = lerp(
            ComplexNumber { x: 0., y: 10. },
            ComplexNumber { x: 10., y: 0. },
            0.3,
        );
        let expected = Ok(ComplexNumber { x: 3., y: 7. });
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_05() {
        let result = lerp(
            ComplexNumber { x: 0., y: 10. },
            ComplexNumber { x: 10., y: 0. },
            -0.5,
        );
        let expected = Err(Error::WrongRangeScalar);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_6() {
        let result = lerp(
            ComplexNumber { x: 0., y: 10. },
            ComplexNumber { x: 10., y: 0. },
            1.1,
        );
        let expected = Err(Error::WrongRangeScalar);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_7() {
        let e1 = Vector::from([
            ComplexNumber { x: 0., y: 10. },
            ComplexNumber { x: 1., y: 0. },
        ]);
        let e2 = Vector::from([
            ComplexNumber { x: 10., y: 0. },
            ComplexNumber { x: 0., y: 1. },
        ]);

        let result = lerp(e1, e2, 0.5);
        let expected = Ok(Vector::from([
            ComplexNumber { x: 5., y: 5. },
            ComplexNumber { x: 0.5, y: 0.5 },
        ]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_8() {
        let e1 = Matrix::from([
            [
                ComplexNumber { x: 0., y: 10. },
                ComplexNumber { x: 1., y: 0. },
            ],
            [
                ComplexNumber { x: 0., y: 10. },
                ComplexNumber { x: 0., y: 0. },
            ],
        ]);
        let e2 = Matrix::from([
            [
                ComplexNumber { x: 10., y: 20. },
                ComplexNumber { x: 1., y: 0. },
            ],
            [
                ComplexNumber { x: 10., y: 0. },
                ComplexNumber { x: 10., y: 10. },
            ],
        ]);
        let result = lerp(e1, e2, 0.5);
        let expected = Ok(Matrix::from([
            [
                ComplexNumber { x: 5., y: 15. },
                ComplexNumber { x: 1., y: 0. },
            ],
            [
                ComplexNumber { x: 5., y: 5. },
                ComplexNumber { x: 5., y: 5. },
            ],
        ]));
        assert_eq!(result, expected);
    }
}
