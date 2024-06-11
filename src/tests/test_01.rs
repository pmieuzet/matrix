#[cfg(test)]
mod test_vector_00 {
    use crate::vector::Vector;
    use std::ops::{Add, Mul, Sub};

    #[test]
    fn test_00_add_01() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        let result = u.add(v);
        let expected = Vector::from([7., 10.]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_add_02() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([0., 0.]);
        let result = u.add(v);
        let expected = Vector::from([0., 0.]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_add_03() {
        let u = Vector::from([-1., 1.]);
        let v = Vector::from([-9., 9.]);
        let result = u.add(v);
        let expected = Vector::from([-10., 10.]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_add_04() {
        let u = Vector::from([2.02, 3.03]);
        let v = Vector::from([5.55, 7.7]);
        let result = u.add(v);
        let expected = Vector::from([7.57, 10.73]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_sub_01() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        let result = u.sub(v);
        let expected = Vector::from([-3., -4.]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_sub_02() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([5., -5.]);
        let result = u.sub(v);
        let expected = Vector::from([-5., 5.]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_sub_03() {
        let u = Vector::from([-5., -5.]);
        let v = Vector::from([-5., 5.]);
        let result = u.sub(v);
        let expected = Vector::from([0., -10.]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_sub_04() {
        let u = Vector::from([2.02, 3.03]);
        let v = Vector::from([5.55, 7.7]);
        let result = u.sub(v);
        let expected = Vector::from([-3.53, -4.67]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_01() {
        let u = Vector::from([2., 3.]);
        let result = u.mul(2.);
        let expected = Vector::from([4., 6.]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_03() {
        let u = Vector::from([0., 0.]);
        let result = u.mul(2.);
        let expected = Vector::from([0., 0.]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_04() {
        let u = Vector::from([2., 3.]);
        let result = u.mul(0.);
        let expected = Vector::from([0., 0.]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_05() {
        let u = Vector::from([2., 3.]);
        let result = u.mul(-2.);
        let expected = Vector::from([-4., -6.]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_06() {
        let u = Vector::from([-2., -3.]);
        let result = u.mul(-2.);
        let expected = Vector::from([4., 6.]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_07() {
        let u = Vector::from([2., 3.03]);
        let result = u.mul(2.22);
        let expected = Vector::from([4.44, 6.7266]);
        assert_eq![result, expected];
    }
}

#[cfg(test)]
mod test_matrix_01 {
    use crate::matrix::Matrix;
    use std::ops::{Add, Mul, Sub};

    #[test]
    fn test_00_add_01() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        let result = u.add(v);
        let expected = Matrix::from([[8., 6.], [1., 6.]]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_add_02() {
        let u = Matrix::from([[-1., -2.], [-3., 4.]]);
        let v = Matrix::from([[0., -2.], [2., -2.]]);
        let result = u.add(v);
        let expected = Matrix::from([[-1., -4.], [-1., 2.]]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_add_03() {
        let u = Matrix::from([[1.1, 2.0], [3.2, 4.2]]);
        let v = Matrix::from([[7.7, 4.4], [-2.2, 2.2]]);
        let result = u.add(v);
        let expected = Matrix::from([[8.8, 6.4], [1.0, 6.4]]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_sub_01() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        let result = u.sub(v);
        let expected = Matrix::from([[-6.0, -2.0], [5.0, 2.0]]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_sub_02() {
        let u = Matrix::from([[1., -2.], [3., -4.]]);
        let v = Matrix::from([[-1., -2.], [-2., 2.]]);
        let result = u.sub(v);
        let expected = Matrix::from([[2.0, 0.0], [5.0, -6.0]]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_sub_03() {
        let u = Matrix::from([[1.1, 2.4], [3.2, 4.2]]);
        let v = Matrix::from([[7.7, 4.0], [-2.2, 2.2]]);
        let result = u.sub(v);
        let expected = Matrix::from([[-6.6, -1.6], [5.4, 2.0]]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_01() {
        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let result = u.mul(2.);
        let expected = Matrix::from([[2.0, 4.0], [6.0, 8.0]]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_02() {
        let u = Matrix::from([[-1., 0.], [-3., -2.]]);
        let result = u.mul(2.);
        let expected = Matrix::from([[-2., 0.], [-6., -4.]]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_03() {
        let u = Matrix::from([[-1., 0.], [-3., 2.]]);
        let result = u.mul(-2.);
        let expected = Matrix::from([[2., 0.], [6., -4.]]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_mul_04() {
        let u = Matrix::from([[-1., 0.], [-3., 2.]]);
        let result = u.mul(0.);
        let expected = Matrix::from([[0., 0.], [0., 0.]]);
        assert_eq![result, expected];
    }
    #[test]
    fn test_00_sub_05() {
        let u = Matrix::from([[1.0, 2.4], [3.0, 4.01]]);
        let result = u.mul(2.22);
        let expected = Matrix::from([[2.22, 5.328], [6.66, 8.9022]]);
        assert_eq![result, expected];
    }
}
