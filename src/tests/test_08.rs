#[cfg(test)]
mod test_08 {
    use crate::errors::Error;
    use crate::matrix::Matrix;

    #[test]
    fn test_08_01() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);

        let result = u.trace();
        let expected = Ok(2.);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_08_02() {
        let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);

        let result = u.trace();
        let expected = Ok(9.);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_08_03() {
        let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);

        let result = u.trace();
        let expected = Ok(-21.);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_08_04() {
        let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.]]);

        let result = u.trace();
        let expected = Err(Error::NotSquareMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_08_05() {
        let u = Matrix::<f32>::from([[]]);

        let result = u.trace();
        let expected = Err(Error::EmptyMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_08_06() {
        let u = Matrix::<f32>::from([[0., 2., 3.], [1., 0., 3.], [1., 2., 0.]]);

        let result = u.trace();
        let expected = Ok(0.);

        assert_eq!(result, expected);
    }
}
