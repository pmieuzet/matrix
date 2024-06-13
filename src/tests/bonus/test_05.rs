#[cfg(test)]
mod test_05 {
    use crate::complex_number::ComplexNumber;
    use crate::errors::Error;
    use crate::{functions::angle_cos, vector::Vector};

    #[test]
    fn test_05_01() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 0. },
            ComplexNumber { x: 0., y: 0. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 1., y: 0. },
            ComplexNumber { x: 0., y: 0. },
        ]);

        let result = angle_cos(&u, &v);
        let expected = Ok(1.);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_05_02() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 0. },
            ComplexNumber { x: 2., y: 0. },
            ComplexNumber { x: 3., y: 0. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 4., y: 0. },
            ComplexNumber { x: 5., y: 0. },
            ComplexNumber { x: 6., y: 0. },
        ]);

        let result = angle_cos(&u, &v);
        let expected = Ok(0.9746318);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_05_03() {
        let u = Vector::<f32>::from([]);
        let v = Vector::<f32>::from([]);

        let result = angle_cos(&u, &v);
        let expected = Err(Error::EmptyVector);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_05_04() {
        let u = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
        ]);
        let v = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
        ]);
        let result = angle_cos(&u, &v);
        let expected = Err(Error::DivisionByZero);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_05_05() {
        let u = Vector::from([ComplexNumber { x: 2., y: 2. }]);
        let v = Vector::from([ComplexNumber { x: 3., y: 0. }]);
        let result = angle_cos(&u, &v);
        let expected = Err(Error::NotImaginaryPartOfComplexNumber);
        assert_eq!(result, expected);
    }
}
