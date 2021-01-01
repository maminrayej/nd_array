use std::{
    borrow::Cow,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::Array;

impl<'a, T: Clone + Neg<Output = T>, const D: usize> Neg for Array<'a, T, D> {
    type Output = Array<'a, T, D>;

    fn neg(mut self) -> Self::Output {
        for idx in 0..self.vec.len() {
            self.vec.to_mut()[idx] = -(self.vec[idx].clone());
        }

        self
    }
}

impl<'a, 'b, T: Clone + Add<Output = T>, const D: usize> Add<&Array<'b, T, D>>
    for &Array<'a, T, D>
{
    type Output = Array<'a, T, D>;

    fn add(self, rhs: &Array<'b, T, D>) -> Self::Output {
        (0..D).for_each(|axis| {
            if self.shape[axis] != rhs.shape[axis] {
                panic!(
                    "Shape of array at axis: {} is not equall to right hand side: {} != {}",
                    axis, self.shape[axis], rhs.shape[axis]
                )
            }
        });

        let sum_vec = self
            .flat()
            .zip(rhs.flat())
            .map(|(v1, v2)| v1.clone() + v2.clone())
            .collect();

        Array::init(sum_vec, self.shape().clone())
    }
}

impl<'a, 'b, T: Clone + Add<Output = T>, const D: usize> Add<Array<'b, T, D>> for Array<'a, T, D> {
    type Output = Array<'a, T, D>;

    fn add(self, rhs: Array<'b, T, D>) -> Self::Output {
        &self + &rhs
    }
}

impl<'a, 'b, T: Clone + Sub<Output = T>, const D: usize> Sub<&Array<'b, T, D>>
    for &Array<'a, T, D>
{
    type Output = Array<'a, T, D>;

    fn sub(self, rhs: &Array<'b, T, D>) -> Self::Output {
        (0..D).for_each(|axis| {
            if self.shape[axis] != rhs.shape[axis] {
                panic!(
                    "Shape of array at axis: {} is not equall to right hand side: {} != {}",
                    axis, self.shape[axis], rhs.shape[axis]
                )
            }
        });

        let sum_vec = self
            .flat()
            .zip(rhs.flat())
            .map(|(v1, v2)| v1.clone() - v2.clone())
            .collect();

        Array::init(sum_vec, self.shape().clone())
    }
}

impl<'a, 'b, T: Clone + Sub<Output = T>, const D: usize> Sub<Array<'b, T, D>> for Array<'a, T, D> {
    type Output = Array<'a, T, D>;

    fn sub(self, rhs: Array<'b, T, D>) -> Self::Output {
        &self - &rhs
    }
}

impl<'a, U: Clone, O: 'a + Clone, T: Clone + Mul<U, Output = O>, const D: usize> Mul<U>
    for &Array<'a, T, D>
{
    type Output = Array<'a, O, D>;

    fn mul(self, rhs: U) -> Self::Output {
        let vec: Vec<O> = self
            .vec
            .iter()
            .map(|val| val.clone() * rhs.clone())
            .collect();

        Array {
            vec: Cow::from(vec),
            shape: self.shape().clone(),
            strides: self.strides().clone(),
            idx_maps: self.idx_maps.clone(),
        }
    }
}

impl<'a, U: Clone, O: 'a + Clone, T: Clone + Mul<U, Output = O>, const D: usize> Mul<U>
    for Array<'a, T, D>
{
    type Output = Array<'a, O, D>;

    fn mul(self, rhs: U) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl<'a, U: Clone, O: 'a + Clone, T: Clone + Div<U, Output = O>, const D: usize> Div<U>
    for &Array<'a, T, D>
{
    type Output = Array<'a, O, D>;

    fn div(self, rhs: U) -> Self::Output {
        let vec: Vec<O> = self
            .vec
            .iter()
            .map(|val| val.clone() / rhs.clone())
            .collect();

        Array {
            vec: Cow::from(vec),
            shape: self.shape().clone(),
            strides: self.strides().clone(),
            idx_maps: self.idx_maps.clone(),
        }
    }
}

impl<'a, U: Clone, O: 'a + Clone, T: Clone + Div<U, Output = O>, const D: usize> Div<U>
    for Array<'a, T, D>
{
    type Output = Array<'a, O, D>;

    fn div(self, rhs: U) -> Self::Output {
        (&self).div(rhs)
    }
}
