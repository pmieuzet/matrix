#[cfg(test)]
mod test_03 {
    use crate::complex_number::ComplexNumber;
    use crate::errors::Error;
    use crate::vector::Vector;

    #[test]
    fn test_03_01() {
        let u = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 1., y: 0. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 0., y: 1. },
        ]);

        let result = u.dot(&v);
        let expected = Ok(ComplexNumber { x: 0., y: 1. });
        assert_eq!(result, expected);
    }
    #[test]
    fn test_03_03() {
        let u = Vector::from([
            ComplexNumber { x: 2., y: 0. },
            ComplexNumber { x: 1., y: 10. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 0., y: 1. },
        ]);

        let result = u.dot(&v);
        let expected = Ok(ComplexNumber { x: 12., y: 3. });
        assert_eq!(result, expected);
    }
    #[test]
    fn test_03_05() {
        let u = Vector::from([ComplexNumber { x: 2., y: 0. }]);
        let v = Vector::from([
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 0., y: 1. },
        ]);

        let result = u.dot(&v);
        let expected = Err(Error::NotSameSize);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_03_06() {
        let u = Vector::<ComplexNumber<f32>>::from([]);
        let v = Vector::<ComplexNumber<f32>>::from([]);

        let result = u.dot(&v);
        let expected = Err(Error::EmptyVector);
        assert_eq!(result, expected);
    }
}
