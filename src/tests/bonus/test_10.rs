#[cfg(test)]
mod test_10 {
    use crate::matrix::Matrix;

    #[test]
    fn test_10_01() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);

        let result = u.row_echelon();
        let expected = Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_10_02() {
        let u: Matrix<f32> = Matrix::from([[1., 2.], [3., 4.]]);

        let result = u.row_echelon();
        let expected = Matrix::from([[1.0, 0.0], [0.0, 1.0]]);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_10_03() {
        let u = Matrix::from([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);

        let result = u.row_echelon();
        let expected = Matrix::from([
            [1.0, 0.625, 0.0, 0.0, -12.166668],
            [0.0, 0.0, 1.0, 0.0, -3.6666667],
            [0.0, 0.0, 0.0, 1.0, 29.5],
        ]);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_10_04() {
        let u = Matrix::from([[-9.]]);

        let result = u.row_echelon();
        let expected = Matrix::from([[1.0]]);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_10_05() {
        let u = Matrix::<f32>::from([[]]);

        let result = u.row_echelon();
        let expected = Matrix::<f32>::from([[]]);

        assert_eq!(result, expected);
    }
}
