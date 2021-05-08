mod access;
mod calc;
mod iter;
mod ops;
mod transformation;

use std::borrow::Cow;

use num_traits::{One, Zero};

#[derive(Debug, Clone, Copy)]
struct IdxMap {
    m: isize,
    b: isize,
}

impl IdxMap {
    fn init() -> Self {
        IdxMap { m: 1, b: 0 }
    }

    fn map(&self, idx: usize) -> usize {
        (self.m * (idx as isize) + self.b) as usize
    }

    fn append_b(&mut self, b: isize) {
        self.b += self.m * b;
    }
}

pub struct Array<'a, T: Clone, const D: usize> {
    vec: Cow<'a, [T]>,
    shape: [usize; D],
    strides: [usize; D],
    idx_maps: [IdxMap; D],
}

impl<'a, T: Clone, const D: usize> Array<'a, T, D> {
    pub fn init(vec: Vec<T>, shape: [usize; D]) -> Self {
        let elem_count: usize = shape.iter().product();

        if elem_count != vec.len() {
            panic!(
                "Number of elements in vec is not equal to dimension specification: {} != {}",
                vec.len(),
                elem_count
            );
        }

        let mut strides = [0; D];
        for axis in 0..D {
            strides[axis] = shape[axis + 1..].iter().fold(1, |acc, v| acc * v);
        }

        Array {
            vec: Cow::from(vec),
            shape,
            strides,
            idx_maps: [IdxMap::init(); D],
        }
    }

    pub fn shape(&self) -> &[usize; D] {
        &self.shape
    }

    pub fn strides(&self) -> &[usize; D] {
        &self.strides
    }

    pub fn full(val: T, shape: [usize; D]) -> Array<'a, T, D> {
        Array::init(vec![val; shape.iter().product()], shape)
    }

    pub fn full_like<'b, U: Clone>(val: T, array: &Array<'b, U, D>) -> Array<'a, T, D> {
        Array::full(val, array.shape().clone())
    }
}

impl<'a, T: Clone> Array<'a, T, 1> {
    pub fn arange<I: Iterator<Item = T>>(range: I) -> Array<'a, T, 1> {
        let vec: Vec<T> = range.collect();
        let len = vec.len();

        Array::init(vec, [len])
    }
}

impl<'a, T: Clone + Zero, const D: usize> Array<'a, T, D> {
    pub fn zeros(shape: [usize; D]) -> Self {
        Array::init(vec![T::zero(); shape.iter().product()], shape)
    }

    pub fn zeros_like<'b, U: Clone>(array: &Array<'b, U, D>) -> Array<'a, T, D> {
        Array::zeros(array.shape().clone())
    }
}

impl<'a, T: Clone + One, const D: usize> Array<'a, T, D> {
    pub fn ones(shape: [usize; D]) -> Self {
        Array::init(vec![T::one(); shape.iter().product()], shape)
    }

    pub fn ones_like<'b, U: Clone>(array: &Array<'b, U, D>) -> Array<'a, T, D> {
        Array::ones(array.shape().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arange() {
        let array = Array::arange(0..10);

        assert_eq!(
            array.flat().copied().collect::<Vec<usize>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        )
    }

    #[test]
    fn zeros() {
        let array = Array::zeros([2, 4]);

        assert_eq!(
            array.flat().copied().collect::<Vec<usize>>(),
            vec![0, 0, 0, 0, 0, 0, 0, 0]
        )
    }

    #[test]
    fn zeros_like() {
        let array = Array::arange(0..8).reshape([2, 4]);

        let zeros_like = Array::zeros_like(&array);

        assert_eq!(
            zeros_like.flat().copied().collect::<Vec<usize>>(),
            vec![0, 0, 0, 0, 0, 0, 0, 0]
        )
    }

    #[test]
    fn ones() {
        let array = Array::ones([2, 4]);

        assert_eq!(
            array.flat().copied().collect::<Vec<usize>>(),
            vec![1, 1, 1, 1, 1, 1, 1, 1]
        )
    }

    #[test]
    fn ones_like() {
        let array = Array::arange(0..8).reshape([2, 4]);

        let ones_like = Array::ones_like(&array);

        assert_eq!(
            ones_like.flat().copied().collect::<Vec<usize>>(),
            vec![1, 1, 1, 1, 1, 1, 1, 1]
        )
    }

    #[test]
    fn full() {
        let array = Array::full(10, [2, 4]);

        assert_eq!(
            array.flat().copied().collect::<Vec<usize>>(),
            vec![10, 10, 10, 10, 10, 10, 10, 10]
        )
    }

    #[test]
    fn full_like() {
        let array = Array::arange(0..8).reshape([2, 4]);

        let full_like = Array::full_like(10, &array);

        assert_eq!(
            full_like.flat().copied().collect::<Vec<usize>>(),
            vec![10, 10, 10, 10, 10, 10, 10, 10]
        )
    }
}
