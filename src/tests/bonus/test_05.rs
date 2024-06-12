#[cfg(test)]
mod test_05 {
    use crate::errors::Error;
    use crate::{functions::angle_cos, vector::Vector};

    #[test]
    fn test_05_01() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);

        let result = angle_cos(&u, &v);
        let expected = Ok(1.);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_05_02() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);

        let result = angle_cos(&u, &v);
        let expected = Ok(0.);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_05_03() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([-1., 0.]);

        let result = angle_cos(&u, &v);
        let expected = Ok(-1.);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_05_04() {
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);

        let result = angle_cos(&u, &v);
        let expected = Ok(0.9746318);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_05_05() {
        let u = Vector::from([1., 2.]);
        let v = Vector::from([4., 5., 6.]);

        let result = angle_cos(&u, &v);
        let expected = Err(Error::NotSameSize);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_05_06() {
        let u = Vector::<f32>::from([]);
        let v = Vector::<f32>::from([]);

        let result = angle_cos(&u, &v);
        let expected = Err(Error::EmptyVector);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_05_07() {
        let u = Vector::from([0., 0.]);
        let v = Vector::<f32>::from([0., 0.]);

        let result = angle_cos(&u, &v);
        let expected = Err(Error::DivisionByZero);
        assert_eq!(result, expected);
    }
}
