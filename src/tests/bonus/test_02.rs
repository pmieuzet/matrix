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
    fn test_02_06() {
        let result = lerp(
            ComplexNumber { x: 0., y: 10. },
            ComplexNumber { x: 10., y: 0. },
            0.3,
        );
        let expected = Ok(ComplexNumber { x: 3., y: 7. });
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_09() {
        let result = lerp(
            ComplexNumber { x: 0., y: 10. },
            ComplexNumber { x: 10., y: 0. },
            -0.5,
        );
        let expected = Err(Error::WrongRangeScalar);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_10() {
        let result = lerp(
            ComplexNumber { x: 0., y: 10. },
            ComplexNumber { x: 10., y: 0. },
            1.1,
        );
        let expected = Err(Error::WrongRangeScalar);
        assert_eq!(result, expected);
    }
    // #[test]
    // fn test_02_11() {
    //     let e1 = Vector::from([
    //         ComplexNumber { x: 0., y: 10. },
    //         ComplexNumber { x: 1., y: 2. },
    //     ]);
    //     let e2 = Vector::from([
    //         ComplexNumber { x: 10., y: 0. },
    //         ComplexNumber { x: 0., y: 10. },
    //     ]);

    //     let result = lerp(e1, e2, 0.5);
    //     let expected = Ok(Vector::from([
    //         ComplexNumber { x: 0., y: 10. },
    //         ComplexNumber { x: 0., y: 10. },
    //     ]));
    //     assert_eq!(result, expected);
    // }
    // #[test]
    // fn test_02_12() {
    //     let e1 = Matrix::from([[2., 1.], [3., 4.]]);
    //     let e2 = Matrix::from([[20., 10.], [30., 40.]]);

    //     let result = lerp(e1, e2, 0.5);
    //     let expected = Ok(Matrix::from([[11., 5.5], [16.5, 22.]]));
    //     assert_eq!(result, expected);
    // }
}
