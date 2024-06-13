#[cfg(test)]
mod test_06 {
    use crate::complex_number::ComplexNumber;
    use crate::errors::Error;
    use crate::{functions::cross_product, vector::Vector};

    #[test]
    fn test_06_01() {
        let u = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 1., y: 1. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 1., y: 0. },
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
        ]);

        let result = cross_product(&u, &v);
        let expected = Ok(Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 0., y: 0. },
        ]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_06_02() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 2., y: 2. },
            ComplexNumber { x: 3., y: 6. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 4., y: 1. },
            ComplexNumber { x: 5., y: 4. },
            ComplexNumber { x: 6., y: 0. },
        ]);

        let result = cross_product(&u, &v);
        let expected = Ok(Vector::from([
            ComplexNumber { x: -27., y: -30. },
            ComplexNumber { x: 12., y: 21. },
            ComplexNumber { x: -1., y: -1. },
        ]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_06_03() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 2., y: 2. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 4., y: 1. },
            ComplexNumber { x: 5., y: 4. },
            ComplexNumber { x: 6., y: 0. },
        ]);

        let result = cross_product(&u, &v);
        let expected = Err(Error::VecNotTridimensional);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_06_04() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 1. },
            ComplexNumber { x: 2., y: 2. },
            ComplexNumber { x: 3., y: 6. },
            ComplexNumber { x: 3., y: 6. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 4., y: 1. },
            ComplexNumber { x: 5., y: 4. },
            ComplexNumber { x: 6., y: 0. },
            ComplexNumber { x: 6., y: 0. },
        ]);

        let result = cross_product(&u, &v);
        let expected = Err(Error::VecNotTridimensional);
        assert_eq!(result, expected);
    }
}
