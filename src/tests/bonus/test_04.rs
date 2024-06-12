#[cfg(test)]
mod test_norm_04 {
    use crate::vector::Vector;

    #[test]
    fn test_04_01() {
        let u = Vector::from([0., 0., 0.]);
        let result = u.norm();
        assert_eq!(result, 0.);
    }
    #[test]
    fn test_04_02() {
        let u = Vector::from([1., 2., 3.]);
        let result = u.norm();
        assert_eq!(result, 3.74165738);
    }
    #[test]
    fn test_04_03() {
        let u = Vector::from([-1., -2.]);
        let result = u.norm();
        assert_eq!(result, 2.236067977);
    }
}

#[cfg(test)]
mod test_norm_1_04 {
    use crate::vector::Vector;

    #[test]
    fn test_04_01() {
        let u = Vector::from([0., 0., 0.]);
        let result = u.norm_1();
        assert_eq!(result, 0.);
    }
    #[test]
    fn test_04_02() {
        let u = Vector::from([1., 2., 3.]);
        let result = u.norm_1();
        assert_eq!(result, 6.);
    }
    #[test]
    fn test_04_03() {
        let u = Vector::from([-1., -2.]);
        let result = u.norm_1();
        assert_eq!(result, 3.);
    }
}

#[cfg(test)]
mod test_norm_inf_04 {
    use crate::vector::Vector;

    #[test]
    fn test_04_01() {
        let u = Vector::from([0., 0., 0.]);
        let result = u.norm_inf();
        assert_eq!(result, 0.);
    }
    #[test]
    fn test_04_02() {
        let u = Vector::from([1., 2., 3.]);
        let result = u.norm_inf();
        assert_eq!(result, 3.);
    }
    #[test]
    fn test_04_03() {
        let u = Vector::from([-1., -2.]);
        let result = u.norm_inf();
        assert_eq!(result, 2.);
    }
}
