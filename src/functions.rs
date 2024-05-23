use std::ops::{Add, Div, Mul, Sub};

use crate::{complex_number::RealNumber, errors::Error, vector::Vector};

/// Creation of a new vector by multiplying each vector by a corresponding scalar, then adding the results
pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Result<Vector<K>, Error>
where
    K: Clone + std::ops::Mul<Output = K> + std::ops::Add<Output = K>,
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

/// Linear interpolation: estimate the value of a function between two given points.
pub fn lerp<V>(u: V, v: V, t: f32) -> Result<V, Error>
where
    V: std::ops::Add<Output = V>
        + std::ops::Mul<f32, Output = V>
        + std::ops::Sub<Output = V>
        + Clone,
{
    if t < 0. || t > 1. {
        return Err(Error::WrongRangeScalar);
    }

    Ok(u.clone() + (v - u) * t)
}

/// compute the cosine of the angle between the two vectors u and v
pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> Result<f32, Error>
where
    K: RealNumber + Mul<Output = K> + Add<Output = K> + Div<f32, Output = f32>,
{
    if u.size != v.size {
        return Err(Error::NotSameSize);
    }

    match u.dot(v) {
        Err(e) => return Err(e),
        Ok(s) => return Ok(s / (u.norm() * v.norm())),
    };
}

/// Compute the cross product of two 3-dimensional vectors
pub fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Result<Vector<K>, Error>
where
    K: Mul<Output = K> + Sub<Output = K> + Copy,
{
    if u.size != 3 || v.size != 3 {
        return Err(Error::VecNotTridimensional);
    }

    let mut data = vec![];
    data.push(u.data[1] * v.data[2] - u.data[2] * v.data[1]);
    data.push(u.data[2] * v.data[0] - u.data[0] * v.data[2]);
    data.push(u.data[0] * v.data[1] - u.data[1] * v.data[0]);

    Ok(Vector { size: 3, data })
}
