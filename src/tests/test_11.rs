#[cfg(test)]
mod test_11 {
    use crate::errors::Error;
    use crate::matrix::Matrix;

    #[test]
    fn test_11_01() {
        let u = Matrix::from([[1., -1.], [-1., 1.]]);

        let result = u.determinant();
        let expected = Ok(0.);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_11_02() {
        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);

        let result = u.determinant();
        let expected = Ok(-174.);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_11_03() {
        let u = Matrix::from([
            [8., 5., -2., 4.],
            [4., 2.5, 20., 4.],
            [8., 5., 1., 4.],
            [28., -4., 17., 1.],
        ]);

        let result = u.determinant();
        let expected = Ok(1032.);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_11_04() {
        let u = Matrix::from([[8., 5., -2., 4.], [4., 2.5, 20., 4.], [8., 5., 1., 4.]]);

        let result = u.determinant();
        let expected = Err(Error::NotSquareMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_11_05() {
        let u = Matrix::from([[8., 5., 1., 4.]]);

        let result = u.determinant();
        let expected = Err(Error::NotSquareMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_11_06() {
        let u = Matrix::<f32>::from([[]]);

        let result = u.determinant();
        let expected = Err(Error::NotSquareMatrix);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_11_07() {
        let u = Matrix::from([[1., 2.], [-2., 0.]]);

        let result = u.determinant();
        let expected = Ok(4.);

        assert_eq!(result, expected);
    }
}
