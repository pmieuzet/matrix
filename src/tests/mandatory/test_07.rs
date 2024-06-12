#[cfg(test)]
mod test_07 {
    use std::ops::Mul;

    use crate::errors::Error;
    use crate::{matrix::Matrix, vector::Vector};

    #[test]
    fn test_07_01() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);

        let result = u.mul(v);
        let expected = Ok(Vector::from([4., 2.]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_07_02() {
        let u = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);

        let result = u.mul(v);
        let expected = Ok(Vector::from([8., 4.]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_07_03() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.], [0., 1.]]);

        let result = u.mul(v);
        let expected = Ok(Matrix::from([[1., 0.], [0., 1.]]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_07_04() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);

        let result = u.mul(v);
        let expected = Ok(Matrix::from([[2., 1.], [4., 2.]]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_07_05() {
        let u = Matrix::from([[3., -5.], [6., 8.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);

        let result = u.mul(v);
        let expected = Ok(Matrix::from([[-14., -7.], [44., 22.]]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_07_06() {
        let u = Matrix::from([[1., 0., 3.], [0., 1., 3.]]);
        let v = Vector::from([4., 2.]);

        let result = u.mul(v);
        let expected = Err(Error::WrongSizeMatrix);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_07_07() {
        let u = Matrix::from([[1., 0., 3.], [1., 0., 3.]]);
        let v = Vector::from([0., 0., 0.]);

        let result = u.mul(v);
        let expected = Ok(Vector::from([0., 0.]));

        assert_eq!(result, expected);
    }
    #[test]
    fn test_07_08() {
        let u = Matrix::from([[1.1, 0.1, 3.1], [2.1, 2.1, 2.1]]);
        let v = Vector::from([0., 1.2, 2.2]);

        let result = u.mul(v);
        let expected = Ok(Vector::from([6.94, 7.14]));

        assert_eq!(result, expected);
    }
    #[test]
    fn test_07_09() {
        let u = Matrix::from([[3., -5.], [6., 8.]]);
        let v = Matrix::from([[2., 1.]]);

        let result = u.mul(v);
        let expected = Err(Error::WrongSizeMatrix);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_07_10() {
        let u = Matrix::from([[3., 5.], [6., 8.], [1., 1.]]);
        let v = Matrix::from([[2., -1., 0., 2.], [0., -1., 2., 2.]]);

        let result = u.mul(v);
        let expected = Ok(Matrix::from([
            [6., -8., 10., 16.],
            [12., -14., 16., 28.],
            [2., -2., 2., 4.],
        ]));
        assert_eq!(result, expected);
    }
}
