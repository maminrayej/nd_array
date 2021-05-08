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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neg() {
        let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

        let neg_array = -array;

        assert_eq!(
            neg_array.flat().copied().collect::<Vec<i32>>(),
            vec![-1, -2, -3, -4, -5, -6]
        );
    }

    #[test]
    fn add() {
        let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);
        let array2 = Array::init(vec![6, 5, 4, 3, 2, 1], [2, 3]);

        let sum_array = array + array2;

        assert_eq!(
            sum_array.flat().copied().collect::<Vec<i32>>(),
            vec![7, 7, 7, 7, 7, 7]
        );
    }

    #[test]
    fn sub() {
        let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);
        let array2 = Array::init(vec![6, 5, 4, 3, 2, 1], [2, 3]);

        let sub_array = array - array2;

        assert_eq!(
            sub_array.flat().copied().collect::<Vec<i32>>(),
            vec![-5, -3, -1, 1, 3, 5]
        );
    }

    #[test]
    fn mul() {
        let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

        let mul_array = array * 2;

        assert_eq!(
            mul_array.flat().copied().collect::<Vec<i32>>(),
            vec![2, 4, 6, 8, 10, 12]
        );
    }

    #[test]
    fn div() {
        let array = Array::init(vec![2, 4, 6, 8, 10, 12], [2, 3]);

        let div_array = array / 2;

        assert_eq!(
            div_array.flat().copied().collect::<Vec<i32>>(),
            vec![1, 2, 3, 4, 5, 6]
        );
    }
}
