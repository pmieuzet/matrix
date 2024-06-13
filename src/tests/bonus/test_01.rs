#[cfg(test)]
mod test_01 {
    use crate::complex_number::ComplexNumber;
    use crate::errors::Error;
    use crate::functions::linear_combination;
    use crate::vector::Vector;

    #[test]
    fn test_01_01() {
        let e1 = Vector::from([
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 0., y: 0. },
        ]);
        let e2 = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 1., y: 1. },
        ]);
        let e3 = Vector::from([
            ComplexNumber { x: 1., y: 0. },
            ComplexNumber { x: 0., y: 1. },
        ]);

        let result = linear_combination(
            &[e1, e2, e3],
            &[
                ComplexNumber { x: 10., y: 0. },
                ComplexNumber { x: -2., y: 1. },
                ComplexNumber { x: 10., y: 0. },
            ],
        );
        let expected = Vector::from([
            ComplexNumber { x: 20., y: 10. },
            ComplexNumber { x: -1., y: 9. },
        ]);
        assert_eq!(result, Ok(expected));
    }
    #[test]
    fn test_01_02() {
        let e1 = Vector::from([ComplexNumber { x: 1., y: 1. }]);
        let e2 = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 1., y: 1. },
        ]);

        let result = linear_combination(&[e1, e2], &[ComplexNumber { x: 0., y: 0. }]);
        let expected = Error::NotSameSize;
        assert_eq!(result, Err(expected));
    }
}
