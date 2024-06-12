#[cfg(test)]
mod test_09 {
    use crate::errors::Error;
    use crate::matrix::Matrix;

    #[test]
    fn test_09_01() {
        let u = Matrix::from([[2., -9., 3.], [13., 11., -17.]]);

        let result = u.transpose();
        let expected = Ok(Matrix::from([[2., 13.], [-9., 11.], [3., -17.]]));

        assert_eq!(result, expected);
    }
    #[test]
    fn test_09_02() {
        let u = Matrix::<f32>::from([[]]);

        let result = u.transpose();
        let expected = Err(Error::EmptyMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_09_03() {
        let u = Matrix::<f32>::from([[0., 1.]]);

        let result = u.transpose();
        let expected = Ok(Matrix::from([[0.], [1.]]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_09_04() {
        let u = Matrix::<f32>::from([[0.]]);

        let result = u.transpose();
        let expected = Ok(Matrix::from([[0.]]));
        assert_eq!(result, expected);
    }
}
