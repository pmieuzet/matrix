#[cfg(test)]
mod test_13 {
    use crate::complex_number::ComplexNumber;
    use crate::matrix::Matrix;

    #[test]
    fn test_13_01() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 0. },
                ComplexNumber { x: -1., y: 0. },
            ],
            [
                ComplexNumber { x: -1., y: 0. },
                ComplexNumber { x: 1., y: 0. },
            ],
        ]);
        let result = u.rank();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_13_04() {
        let u = Matrix::from([
            [
                ComplexNumber { x: 1., y: 6. },
                ComplexNumber { x: -1., y: 12. },
                ComplexNumber { x: 6., y: 10. },
            ],
            [
                ComplexNumber { x: -1., y: 8. },
                ComplexNumber { x: 1., y: 9. },
                ComplexNumber { x: 0., y: 0. },
            ],
        ]);
        let result = u.rank();
        assert_eq!(result, 2);
    }
    #[test]
    fn test_13_05() {
        let u: Matrix<ComplexNumber<f32>> = Matrix::from([[]]);

        let result = u.rank();
        assert_eq!(result, 0);
    }
}
