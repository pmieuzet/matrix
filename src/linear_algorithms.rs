use std::ops::{Add, Mul};

use crate::{errors::Error, matrix::Matrix, vector::Vector};

/// Creation of a new vector by multiplying each vector by a corresponding scalar, then adding the results
pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Result<Vector<K>, Error>
where
    K: std::ops::AddAssign + Clone + std::ops::Mul<Output = K> + std::ops::Add<Output = K>,
{
    if u.len() != coefs.len() {
        return Err(Error::NotSameSize);
    }

    u.to_owned()
        .into_iter()
        .zip(coefs.to_owned().into_iter())
        .map(|(a, coef)| a.mul(coef))
        .reduce(|acc, a| acc.add(a))
        .ok_or(Error::EmptyVector)
}

pub fn lerp<V>(u: V, v: V, t: f32) -> Result<V, Error>
where
    V: std::ops::Add<Output = V>
        + std::ops::Mul<f32, Output = V>
        + std::ops::Sub<Output = V>
        + std::ops::Div<f32, Output = V>
        + Clone,
{
    if t < 0. || t > 1. {
        return Err(Error::WrongRangeScalar);
    }

    match t {
        0.0 => Ok(u),
        1.0 => Ok(v),
        0.5 => Ok((u + v).div(2.)),
        _ => Ok(v - u.clone() + u.mul(t)),
    }
}
