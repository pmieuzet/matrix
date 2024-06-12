#[cfg(test)]
mod test_02 {
    use crate::errors::Error;
    use crate::functions::lerp;
    use crate::matrix::Matrix;
    use crate::vector::Vector;

    #[test]
    fn test_02_01() {
        let result = lerp(0., 1., 0.);
        let expected = Ok(0.);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_02() {
        let result = lerp(0., 1., 1.);
        let expected = Ok(1.);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_03() {
        let result = lerp(0., 1., 0.5);
        let expected = Ok(0.5);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_04() {
        let result = lerp(21., 42., 0.3);
        let expected = Ok(27.3);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_05() {
        let result = lerp(1., 0., 0.5);
        let expected = Ok(0.5);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_06() {
        let result = lerp(1., 0., 1.);
        let expected = Ok(0.);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_07() {
        let result = lerp(0., 0., 1.);
        let expected = Ok(0.);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_08() {
        let result = lerp(42., 21., 0.3);
        let expected = Ok(35.7);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_09() {
        let result = lerp(42., 21., -0.5);
        let expected = Err(Error::WrongRangeScalar);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_10() {
        let result = lerp(42., 21., 1.1);
        let expected = Err(Error::WrongRangeScalar);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_11() {
        let e1 = Vector::from([2., 1.]);
        let e2 = Vector::from([4., 2.]);

        let result = lerp(e1, e2, 0.3);
        let expected = Ok(Vector::from([2.6, 1.3]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_12() {
        let e1 = Matrix::from([[2., 1.], [3., 4.]]);
        let e2 = Matrix::from([[20., 10.], [30., 40.]]);

        let result = lerp(e1, e2, 0.5);
        let expected = Ok(Matrix::from([[11., 5.5], [16.5, 22.]]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_02_13() {
        let e1 = Matrix::from([[2., 1.1], [3., 4.1]]);
        let e2 = Matrix::from([[20., 10.], [30., 40.]]);

        let result = lerp(e1, e2, 0.);
        let expected = Ok(Matrix::from([[2., 1.1], [3., 4.1]]));
        assert_eq!(result, expected);
    }
}
