use crate::matrix;
use matrix::Matrix;

pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    let tangent = f32::tan((fov / 2.) * (f32::acos(-1.) / 180.));
    // let right = f32::tan(fov / 2.) * near;
    let right = near * tangent;
    let top = right * ratio;

    Matrix::from([
        [near / right, 0., 0., 0.],
        [0., near / top, 0., 0.],
        [0., 0., -((far + near) / (far - near)), -1.],
        [0., 0., -((2. * far * near) * (far - near)), 0.],
    ])
}
