use std::ops::{Add, Mul};

use crate::{errors::Error, vector::Vector};

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
