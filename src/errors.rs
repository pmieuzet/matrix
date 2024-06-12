#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("Error: The two arguments are not the same size.")]
    NotSameSize,
    #[error("Error: The vector is empty.")]
    EmptyVector,
    #[error("Error: The scalar is a wrong range.")]
    WrongRangeScalar,
    #[error("Error: The vector is not tridimensional.")]
    VecNotTridimensional,
    #[error("Error: Wrong size of matrix.")]
    WrongSizeMatrix,
    #[error("Error: The matrix must be square.")]
    NotSquareMatrix,
    #[error("Error: The determinant of the matrix must be not null.")]
    NullDeterminantMatrix,
    #[error("Error: The imaginary part is not null")]
    NotImaginaryPartOfComplexNumber,
    #[error("Error: The division by zero is not allowed.")]
    DivisionByZero,
    #[error("Error: The matrix is empty.")]
    EmptyMatrix,
    #[error("Error: The two matrices are not the same size.")]
    NotSameSizeMatrix,
}
