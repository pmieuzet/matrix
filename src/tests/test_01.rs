#[cfg(test)]
mod test_01 {
    use crate::functions::linear_combination;
    use crate::vector::Vector;

    #[test]
    fn test_01_01() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);

        let result = linear_combination(&[e1, e2, e3], &[10., -2., 0.5]);
        let expected = Vector::from([10., -2., 0.5]);
        assert_eq!(result, Ok(expected));
    }
    #[test]
    fn test_01_02() {
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);

        let result = linear_combination(&[v1, v2], &[10., -2.]);
        let expected = Vector::from([10., 0., 230.]);
        assert_eq!(result, Ok(expected));
    }
}
