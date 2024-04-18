#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error: The two arguments are not the same size.")]
    NotSameSize,
    #[error("Error: The vector is empty.")]
    EmptyVector,
    #[error("Error: The scalar is a wrong range.")]
    WrongRangeScalar,
}
