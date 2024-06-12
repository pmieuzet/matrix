#[cfg(test)]
mod test_13 {
    use crate::errors::Error;
    use crate::matrix::Matrix;

    #[test]
    fn test_13_01() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);

        let result = u.rank();
        assert_eq!(result, 3);
    }
    #[test]
    fn test_13_02() {
        let u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);

        let result = u.rank();
        assert_eq!(result, 2);
    }
    #[test]
    fn test_13_03() {
        let u: Matrix<f32> =
            Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);

        let result = u.rank();
        assert_eq!(result, 3);
    }
    #[test]
    fn test_13_04() {
        let u: Matrix<f32> = Matrix::from([[0.]]);

        let result = u.rank();
        assert_eq!(result, 0);
    }
    #[test]
    fn test_13_05() {
        let u: Matrix<f32> = Matrix::from([[]]);

        let result = u.rank();
        assert_eq!(result, 0);
    }
    #[test]
    fn test_13_06() {
        let u: Matrix<f32> = Matrix::from([[1.], [0.]]);

        let result = u.rank();
        assert_eq!(result, 1);
    }
    #[test]
    fn test_13_07() {
        let u: Matrix<f32> = Matrix::from([[1., 0.], [1., 0.]]);

        let result = u.rank();
        assert_eq!(result, 1);
    }
}
