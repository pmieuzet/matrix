use std::ops::Mul;

use crate::vector::Vector;

pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: std::ops::AddAssign + Clone + std::ops::Mul<Output = K>,
{
    let x = u
        .iter()
        .zip(coefs.iter())
        .map(|(a, coef)| (*a).clone().mul((*coef).clone()));

    let mut data = Vec::<K>::new();
    for item in x.into_iter() {
        for i in 0..item.size {
            if data.len() == i {
                data.push(item.data[i].clone());
            } else {
                data[i] += item.data[i].clone();
            }
        }
    }

    Vector {
        size: data.len(),
        data,
    }
}
