use std::ops::{Add, Mul, Sub};

use crate::{
    real_number::{RealNumber},
    errors::Error,
    ops_safe::{AddSafe, DivSafe, MulByf32, SubSafe},
    vector::Vector,
};

/// Creation of a new vector by multiplying each vector by a corresponding scalar, then adding the results.
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
        .reduce(|acc, a| acc.add(a).unwrap())
        .ok_or(Error::EmptyVector)
}

/// This function allows you to find any point on the segment connecting two vectors. 
/// It uses a parameter t (between 0 and 1) that acts as a cursor.
/// If t=0, we are at the start; if t=1, we are at the end; if t=0.5, we are right in the middle.
pub fn lerp<V>(u: V, v: V, t: f32) -> Result<V, Error>
where
    V: AddSafe + MulByf32 + SubSafe + Clone,
{
    if t < 0. || t > 1. {
        return Err(Error::WrongRangeScalar);
    }

    // u + (v - u) * t
    AddSafe::add(u.clone(), MulByf32::mul(SubSafe::sub(v, u)?, t))
}

/// Compute the cosine of the angle between the two vectors u and v.
/// Find the exact angle between two vectors, regardless of their length. We use the dot product to calculate it.
/// This gives us a pure value between -1 and 1 that describes the exact angle between two objects.
pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> Result<f32, Error>
where
    K: DivSafe + RealNumber + Mul<Output = K> + Add<Output = K>,
{
    if u.size() != v.size() {
        return Err(Error::NotSameSize);
    }

    match u.dot(v) {
        Err(e) => Err(e),
        Ok(s) => DivSafe::div(s, u.norm() * v.norm()),
    }
}

/// Compute the cross product of two 3-dimensional vectors
/// This operation takes two vectors and creates a third that is perpendicular to the first two.
pub fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Result<Vector<K>, Error>
where
    K: Mul<Output = K> + Sub<Output = K> + Copy,
{
    if u.size() != 3 || v.size() != 3 {
        return Err(Error::VecNotTridimensional);
    }

    let mut data = vec![];
    data.push(u.data[1] * v.data[2] - u.data[2] * v.data[1]);
    data.push(u.data[2] * v.data[0] - u.data[0] * v.data[2]);
    data.push(u.data[0] * v.data[1] - u.data[1] * v.data[0]);

    Ok(Vector { data })
}
