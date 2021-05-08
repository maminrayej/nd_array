use std::{
    borrow::Cow,
    ops::{Add, Div, Mul, Sub},
};

use num_traits::{FromPrimitive, One, Zero};

use crate::Array;

impl<'a, T: Clone + Ord, const D: usize> Array<'a, T, D> {
    pub fn max(&self) -> Option<T> {
        self.flat().max().cloned()
    }

    pub fn arg_max(&self) -> Vec<usize> {
        let mut positions = vec![];

        if let Some(max) = self.max() {
            for (index, value) in self.flat().enumerate() {
                if value == &max {
                    positions.push(index)
                }
            }
        }

        positions
    }

    pub fn max_across(&self, axis: usize) -> Vec<Option<T>> {
        self.axis_view(axis).map(|view| view.max()).collect()
    }

    pub fn arg_max_across(&self, axis: usize) -> Vec<Option<usize>> {
        self.axis_view(axis)
            .map(|view| view.arg_max().get(0).copied())
            .collect()
    }

    pub fn min(&self) -> Option<T> {
        self.flat().min().cloned()
    }

    pub fn arg_min(&self) -> Vec<usize> {
        let mut positions = vec![];

        if let Some(min) = self.min() {
            for (index, value) in self.flat().enumerate() {
                if value == &min {
                    positions.push(index)
                }
            }
        }

        positions
    }

    pub fn min_across(&self, axis: usize) -> Vec<Option<T>> {
        self.axis_view(axis).map(|view| view.min()).collect()
    }

    pub fn arg_min_across(&self, axis: usize) -> Vec<Option<usize>> {
        self.axis_view(axis)
            .map(|view| view.arg_min().get(0).copied())
            .collect()
    }

    pub fn clip(&self, min: &T, max: &T) -> Array<'a, T, D> {
        let vec: Vec<T> = self
            .vec
            .iter()
            .map(|val| val.clamp(min, max).clone())
            .collect();

        let shape = self.shape.clone();
        let strides = self.strides.clone();
        let idx_maps = self.idx_maps.clone();

        Array {
            vec: Cow::from(vec),
            shape,
            strides,
            idx_maps,
        }
    }
}

impl<'a, T, const D: usize> Array<'a, T, D>
where
    T: Clone + Ord + Sub<Output = T>,
{
    pub fn ptp(&self) -> Option<T> {
        self.max().and_then(|max| self.min().map(|min| max - min))
    }

    pub fn ptp_across(&self, axis: usize) -> Vec<Option<T>> {
        self.axis_view(axis).map(|view| view.ptp()).collect()
    }
}

impl<'a, T, const D: usize> Array<'a, T, D>
where
    T: Clone + Add<Output = T> + Zero,
{
    pub fn sum(&self) -> T {
        self.flat().fold(T::zero(), |acc, val| acc + val.clone())
    }

    pub fn sum_across(&self, axis: usize) -> Vec<T> {
        self.axis_view(axis).map(|view| view.sum()).collect()
    }
}

impl<'a, T, const D: usize> Array<'a, T, D>
where
    T: Clone + Mul<Output = T> + One,
{
    pub fn prod(&self) -> T {
        self.flat().fold(T::one(), |acc, val| acc * val.clone())
    }

    pub fn prod_across(&self, axis: usize) -> Vec<T> {
        self.axis_view(axis).map(|view| view.prod()).collect()
    }
}

impl<'a, T, const D: usize> Array<'a, T, D>
where
    T: Clone + Add<Output = T> + FromPrimitive + Div<T, Output = T> + Zero,
{
    pub fn mean(&self) -> T {
        self.sum() / T::from_usize(self.shape().iter().product()).unwrap()
    }

    pub fn mean_across(&self, axis: usize) -> Vec<T> {
        self.axis_view(axis).map(|view| view.mean()).collect()
    }
}

impl<'a, T, const D: usize> Array<'a, T, D>
where
    T: Clone + Sub<Output = T> + FromPrimitive + Div<T, Output = T> + Mul<Output = T> + Zero,
{
    pub fn var(&self) -> T {
        let mean = self.mean();

        self.flat().fold(T::zero(), |acc, val| {
            acc + (val.clone() - mean.clone()) * (val.clone() - mean.clone())
        }) / T::from_usize(self.shape().iter().product()).unwrap()
    }

    pub fn var_across(&self, axis: usize) -> Vec<T> {
        self.axis_view(axis).map(|view| view.var()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max() {
        // 2-D array:
        // 0 1
        // 2 3
        let array = Array::init(vec![0, 1, 2, 3], [2, 2]);

        assert_eq!(array.max().unwrap(), 3);
    }

    #[test]
    fn arg_max() {
        // 2-D array:
        // 0 1
        // 2 3
        let array = Array::init(vec![0, 1, 2 , 3], [2, 2]);

        assert_eq!(array.arg_max()[0], 3);
    }

    #[test]
    fn max_across() {
        // 2-D array:
        // 0 1
        // 2 3
        let array = Array::init(vec![0, 1, 2, 3], [2, 2]);

        assert_eq!(array.max_across(1), vec![Some(2), Some(3)]);
        assert_eq!(array.max_across(0), vec![Some(1), Some(3)]);
    }

    #[test]
    fn arg_max_across() {
        // 2-D array:
        // 0 1
        // 2 3
        let array = Array::init(vec![0, 1, 2, 3], [2, 2]);

        assert_eq!(array.arg_max_across(1), vec![Some(1), Some(1)]);
        assert_eq!(array.arg_max_across(0), vec![Some(1), Some(1)]);
    }

    #[test]
    fn min() {
        // 2-D array:
        // 0 1
        // 2 3
        let array = Array::init(vec![0, 1, 2, 3], [2, 2]);

        assert_eq!(array.min().unwrap(), 0);
    }

    #[test]
    fn arg_min() {
        // 2-D array:
        // 0 1
        // 2 3
        let array = Array::init(vec![0, 1, 2, 3], [2, 2]);

        assert_eq!(array.arg_min()[0], 0);
    }

    #[test]
    fn min_across() {
        // 2-D array:
        // 0 1
        // 2 3
        let array = Array::init(vec![0, 1, 2, 3], [2, 2]);

        assert_eq!(array.min_across(1), vec![Some(0), Some(1)]);
        assert_eq!(array.min_across(0), vec![Some(0), Some(2)]);
    }

    #[test]
    fn arg_min_across() {
        // 2-D array:
        // 0 1
        // 2 3
        let array = Array::init(vec![0, 1, 2, 3], [2, 2]);

        assert_eq!(array.arg_min_across(1), vec![Some(0), Some(0)]);
        assert_eq!(array.arg_min_across(0), vec![Some(0), Some(0)]);
    }

    #[test]
    fn clip() {
        let array = Array::arange(0..10);

        let clipped = array.clip(&1, &8);

        assert_eq!(
            clipped.flat().copied().collect::<Vec<i32>>(),
            vec![1, 1, 2, 3, 4, 5, 6, 7, 8, 8]
        );
    }

    #[test]
    fn ptp() {
        let array = Array::init(vec![4, 9, 2, 10, 6, 9, 7, 12], [2, 4]);

        assert_eq!(array.ptp().unwrap(), 10)
    }

    #[test]
    fn ptp_across() {
        let array = Array::init(vec![4, 9, 2, 10, 6, 9, 7, 12], [2, 4]);

        assert_eq!(array.ptp_across(0), vec![Some(8), Some(6)]);
        assert_eq!(
            array.ptp_across(1),
            vec![Some(2), Some(0), Some(5), Some(2)]
        )
    }

    #[test]
    fn sum() {
        // 1 2
        // 3 4
        let array = Array::arange(1..5).reshape([2, 2]);

        assert_eq!(array.sum(), 10);
    }

    #[test]
    fn sum_across() {
        // 1 2
        // 3 4
        let array = Array::arange(1..5).reshape([2, 2]);

        assert_eq!(array.sum_across(0), vec![3, 7]);
        assert_eq!(array.sum_across(1), vec![4, 6]);
    }

    #[test]
    fn prod() {
        // 1 2
        // 3 4
        let array = Array::arange(1..5).reshape([2, 2]);

        assert_eq!(array.prod(), 24);
    }

    #[test]
    fn prod_across() {
        // 1 2
        // 3 4
        let array = Array::arange(1..5).reshape([2, 2]);

        assert_eq!(array.prod_across(0), vec![2, 12]);
        assert_eq!(array.prod_across(1), vec![3, 8]);
    }

    #[test]
    fn mean() {
        // 1 2
        // 3 4
        let array = Array::arange(1..5).reshape([2, 2]);

        assert_eq!(array.mean(), 2);
    }

    #[test]
    fn mean_across() {
        // 1 2
        // 3 4
        let array = Array::arange(1..5).reshape([2, 2]);

        assert_eq!(array.mean_across(0), vec![1, 3]);
        assert_eq!(array.mean_across(1), vec![2, 3]);
    }

    #[test]
    fn var() {
        // 1 2
        // 3 4
        let array = Array::init(vec![1.0, 2.0, 3.0, 4.0], [2, 2]);

        assert_eq!(array.var(), 1.25);
    }

    #[test]
    fn var_across() {
        // 1 2
        // 3 4
        let array = Array::init(vec![1.0, 2.0, 3.0, 4.0], [2, 2]);

        assert_eq!(array.var_across(0), vec![0.25, 0.25]);
        assert_eq!(array.var_across(1), vec![1.0, 1.0]);
    }
}
