#[cfg(test)]
mod test_06 {
    use crate::errors::Error;
    use crate::{functions::cross_product, vector::Vector};

    #[test]
    fn test_06_01() {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0., 0.]);

        let result = cross_product(&u, &v);
        let expected = Ok(Vector::from([0., 1., 0.]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_06_02() {
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);

        let result = cross_product(&u, &v);
        let expected = Ok(Vector::from([-3., 6., -3.]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_06_03() {
        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([-2., -5., 16.]);

        let result = cross_product(&u, &v);
        let expected = Ok(Vector::from([17., -58., -16.]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_06_04() {
        let u = Vector::from([4., 2.]);
        let v = Vector::from([-2., -5., 16.]);

        let result = cross_product(&u, &v);
        let expected = Err(Error::VecNotTridimensional);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_06_05() {
        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([1., 1., 1., 1.]);

        let result = cross_product(&u, &v);
        let expected = Err(Error::VecNotTridimensional);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_06_06() {
        let u = Vector::from([0., 1., 0.]);
        let v = Vector::from([1., 1., 1.]);

        let result = cross_product(&u, &v);
        let expected = Ok(Vector::from([1., 0., -1.]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_06_07() {
        let u = Vector::from([1., 1., 1.]);
        let v = Vector::from([0., 0., 0.]);

        let result = cross_product(&u, &v);
        let expected = Ok(Vector::from([0., 0., 0.]));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_06_08() {
        let v = Vector::from([1., 1., 1.]);
        let u = Vector::from([0., 0., 0.]);

        let result = cross_product(&u, &v);
        let expected = Ok(Vector::from([0., 0., 0.]));
        assert_eq!(result, expected);
    }
}
