#[cfg(test)]
mod test_norm_04 {
    use crate::complex_number::ComplexNumber;
    use crate::vector::Vector;

    #[test]
    fn test_04_01() {
        let u = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
        ]);
        let result = u.norm();
        assert_eq!(result, 0.);
    }
    #[test]
    fn test_04_02() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 0. },
            ComplexNumber { x: 0., y: 10. },
            ComplexNumber { x: 3., y: 2. },
        ]);
        let result = u.norm();
        assert_eq!(result, 10.677078);
    }
}

#[cfg(test)]
mod test_norm_1_04 {
    use crate::complex_number::ComplexNumber;
    use crate::vector::Vector;

    #[test]
    fn test_04_01() {
        let u = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
        ]);
        let result = u.norm_1();
        assert_eq!(result, 0.);
    }
    #[test]
    fn test_04_02() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 0. },
            ComplexNumber { x: 0., y: 10. },
            ComplexNumber { x: 3., y: 2. },
        ]);
        let result = u.norm_1();
        assert_eq!(result, 14.605551);
    }
}

#[cfg(test)]
mod test_norm_inf_04 {
    use crate::complex_number::ComplexNumber;
    use crate::vector::Vector;

    #[test]
    fn test_04_01() {
        let u = Vector::from([
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
            ComplexNumber { x: 0., y: 0. },
        ]);
        let result = u.norm_inf();
        assert_eq!(result, 0.);
    }
    #[test]
    fn test_04_02() {
        let u = Vector::from([
            ComplexNumber { x: 1., y: 0. },
            ComplexNumber { x: 0., y: 10. },
            ComplexNumber { x: 3., y: 2. },
        ]);
        let result = u.norm_inf();
        assert_eq!(result, 10.);
    }
}
