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
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);

        let result = linear_combination(&[v1, v2], &[10., -2.]);
        let expected = Vector::from([10., 0., 230.]);
        assert_eq!(result, Ok(expected));
    }
    #[test]
    fn test_01_03() {
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);

        let result = linear_combination(&[v1, v2], &[0.]);
        let expected = Error::NotSameSize;
        assert_eq!(result, Err(expected));
    }
    #[test]
    fn test_01_04() {
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([1., 2., 3.]);

        let result = linear_combination(&[v1, v2], &[1., 1.]);
        let expected = Vector::from([2., 4., 6.]);
        assert_eq!(result, Ok(expected));
    }
    #[test]
    fn test_01_05() {
        let v1 = Vector::from([1.1, 2.6, 3.3]);
        let v2 = Vector::from([1.8, 2.8, 3.3]);

        let result = linear_combination(&[v1, v2], &[0., 1.]);
        let expected = Vector::from([1.8, 2.8, 3.3]);
        assert_eq!(result, Ok(expected));
    }
    #[test]
    fn test_01_06() {
        let v1 = Vector::from([]);
        let v2 = Vector::from([]);

        let result = linear_combination(&[v1, v2], &[0., 1.]);
        let expected = Ok(Vector::from([]));
        assert_eq!(result, expected);
    }
}
