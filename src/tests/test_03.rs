#[cfg(test)]
mod test_03 {
    use crate::errors::Error;
    use crate::vector::Vector;

    #[test]
    fn test_03_01() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([1., 1.]);

        let result = u.dot(&v);
        let expected = Ok(0.);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_03_02() {
        let u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);

        let result = u.dot(&v);
        let expected = Ok(2.);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_03_03() {
        let u = Vector::from([-1., 6.]);
        let v = Vector::from([3., 2.]);

        let result = u.dot(&v);
        let expected = Ok(9.);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_03_04() {
        let u = Vector::from([-1., 6.]);
        let v = Vector::from([-1., -1.]);

        let result = u.dot(&v);
        let expected = Ok(-5.);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_03_05() {
        let u = Vector::from([-1., 6., 0.]);
        let v = Vector::from([-1., -1.]);

        let result = u.dot(&v);
        let expected = Err(Error::NotSameSize);
        assert_eq!(result, expected);
    }
}
