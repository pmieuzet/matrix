#[cfg(test)]
mod test_12 {
    use crate::errors::Error;
    use crate::matrix::Matrix;

    #[test]
    fn test_12_01() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);

        let result = u.inverse();
        let expected = Ok(Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]));

        assert_eq!(result, expected);
    }
    #[test]
    fn test_12_02() {
        let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);

        let result = u.inverse();
        let expected = Ok(Matrix::from([[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]]));

        assert_eq!(result, expected);
    }
    #[test]
    fn test_12_03() {
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);

        let result = u.inverse();
        let expected = Ok(Matrix::from([
            [0.649425287, 0.097701149, -0.655172414],
            [-0.781609195, -0.126436782, 0.965517241],
            [0.143678161, 0.07471265, -0.206896552],
        ]));

        assert_eq!(result, expected);
    }
    #[test]
    fn test_12_04() {
        let u = Matrix::from([[8., 5., -2.]]);

        let result = u.inverse();
        let expected = Err(Error::NotSquareMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_12_05() {
        let u = Matrix::from([[8.]]);

        let result = u.inverse();
        let expected = Err(Error::NotSquareMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_12_06() {
        let u = Matrix::from([[1., -1.], [-1., 1.]]);

        let result = u.inverse();
        let expected = Err(Error::NullDeterminantMatrix);

        assert_eq!(result, expected);
    }
}
