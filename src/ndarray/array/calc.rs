use std::{
    borrow::Cow,
    ops::{Add, Mul, Sub},
};

use num_traits::{One, Zero};

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
