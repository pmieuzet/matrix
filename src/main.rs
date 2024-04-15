use std::fmt::Display;

#[derive(Clone, Debug)]
struct Vector<T> {
    size: usize,
    data: Vec<T>,
}
impl<T> Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
}
impl<T> Vector<T> {
    fn size(&self) -> usize {
        self.size
    }
}

#[derive(Clone, Debug)]
struct Matrix<T> {
    rows: usize,
    colums: usize,
    data: Vec<Vec<T>>,
}
impl<T> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
}
impl<T> Matrix<T> {
    fn is_square() -> bool {
        true
    }
}

fn main() {
    println!("Hello, world!");
}
