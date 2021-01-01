mod access;
mod iter;
mod transformation;
mod calc;

use std::borrow::Cow;

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
        let elem_count = shape.iter().fold(1, |acc, v| acc * v);

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
}
