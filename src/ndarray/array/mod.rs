use std::{borrow::Cow, ops::Index};

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

    pub fn transpose(mut self) -> Array<'a, T, D> {
        self.shape.reverse();
        self.strides.reverse();
        self.idx_maps.reverse();

        self
    }

    pub fn t(&'a self) -> Array<'a, T, D> {
        let mut shape = self.shape.clone();
        let mut strides = self.strides.clone();
        let mut idx_maps = self.idx_maps.clone();

        shape.reverse();
        strides.reverse();
        idx_maps.reverse();

        Array {
            vec: Cow::from(&*self.vec),
            shape,
            strides,
            idx_maps
        }
    }

    pub fn get(&self, indices: [usize; D]) -> Option<&T> {
        if indices
            .iter()
            .enumerate()
            .any(|(axis, idx)| *idx >= self.shape[axis])
        {
            return None;
        }

        let index = indices
            .iter()
            .enumerate()
            .fold(0, |acc, (axis, axis_index)| {
                acc + self.idx_maps[axis].map(*axis_index) * self.strides[axis]
            });

        self.vec.get(index)
    }

    pub fn iter(&self) -> Iter<'_, T, D> {
        Iter::init(self)
    }

    pub fn flip(&'a self, axis: usize) -> Array<'a, T, D> {
        if axis >= D {
            panic!("Axis out of bounds")
        }

        let mut idx_maps = self.idx_maps.clone();

        let idx_map = &mut idx_maps[axis];

        if idx_map.m == -1 {
            idx_map.m = 1;
            idx_map.b = idx_map.b - (self.shape[axis] - 1) as isize
        } else {
            idx_map.m = -1;
            idx_map.b = idx_map.b + (self.shape[axis] - 1) as isize
        }

        Array {
            vec: Cow::from(&*self.vec),
            shape: self.shape.clone(),
            strides: self.strides.clone(),
            idx_maps
        }
    }

    pub fn swap_axes(&'a self, axis0: usize, axis1: usize) -> Array<'a, T, D> {
        if axis0 >= D || axis1 >= D {
            panic!("Axis out of bounds")
        }

        let mut shape = self.shape.clone();
        let mut strides = self.strides.clone();
        let mut idx_maps = self.idx_maps.clone();

        shape.swap(axis0, axis1);
        strides.swap(axis0, axis1);
        idx_maps.swap(axis0, axis1);

        Array {
            vec: Cow::from(&*self.vec),
            shape,
            strides,
            idx_maps
        }
    }

    pub fn reshape<const S: usize>(&self, shape: [usize; S]) -> Array<'a, T, S> {
        // TODO: Check if c-contigous

        let vec = self.iter().cloned().collect();

        Array::init(vec, shape)
    }
}

impl<'a, T: Clone, const D: usize> Index<[usize; D]> for Array<'a, T, D> {
    type Output = T;

    fn index(&self, indices: [usize; D]) -> &Self::Output {
        if indices
            .iter()
            .enumerate()
            .any(|(axis, idx)| *idx >= self.shape[axis])
        {
            panic!("Index out of bound");
        }

        let index = indices
            .iter()
            .enumerate()
            .fold(0, |acc, (axis, axis_index)| {
                acc + self.idx_maps[axis].map(*axis_index) * self.strides[axis]
            });

        &self.vec[index]
    }
}

pub struct Iter<'a, T: Clone, const D: usize> {
    array: &'a Array<'a, T, D>,
    indices: [usize; D],
}

impl<'a, T: Clone, const D: usize> Iter<'a, T, D> {
    fn init(array: &'a Array<'a, T, D>) -> Self {
        Iter {
            array,
            indices: [0; D],
        }
    }

    fn increment_indices(&mut self) {
        self.increment_idx_at_axis(D - 1)
    }

    fn increment_idx_at_axis(&mut self, axis: usize) {
        self.indices[axis] += 1;

        if axis != 0 && self.indices[axis] >= self.array.shape[axis] {
            self.indices[axis] = 0;

            if axis > 0 {
                self.increment_idx_at_axis(axis - 1);
            }
        }
    }
}

impl<'a, T: Clone, const D: usize> Iterator for Iter<'a, T, D> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.array.get(self.indices);

        self.increment_indices();

        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_array() {
        // 2-D array:
        // 1 2 3
        // 4 5 6
        let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

        assert_eq!(array[[0, 0]], 1);
        assert_eq!(array[[0, 1]], 2);
        assert_eq!(array[[0, 2]], 3);
        assert_eq!(array[[1, 0]], 4);
        assert_eq!(array[[1, 1]], 5);
        assert_eq!(array[[1, 2]], 6);
    }

    #[test]
    fn iter() {
        // 2-D array:
        // 1 2 3
        // 4 5 6
        let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

        assert_eq!(
            array.iter().copied().collect::<Vec<usize>>(),
            vec![1, 2, 3, 4, 5, 6]
        )
    }

    #[test]
    fn reshape_array() {
        // 2-D array:
        // 1 2 3
        // 4 5 6
        let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

        // reshape it to the 3x2 2-D array:
        // 1 2
        // 3 4
        // 5 6
        let array = array.reshape([3, 2]);

        assert_eq!(array[[0, 0]], 1);
        assert_eq!(array[[0, 1]], 2);
        assert_eq!(array[[1, 0]], 3);
        assert_eq!(array[[1, 1]], 4);
        assert_eq!(array[[2, 0]], 5);
        assert_eq!(array[[2, 1]], 6);
    }

    #[test]
    fn transpose() {
        // 2-D array:
        // 1 2 3
        // 4 5 6
        let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

        // tranpose the array to:
        // 1 4
        // 2 5
        // 3 6
        let array = array.transpose();

        assert_eq!(array[[0, 0]], 1);
        assert_eq!(array[[0, 1]], 4);
        assert_eq!(array[[1, 0]], 2);
        assert_eq!(array[[1, 1]], 5);
        assert_eq!(array[[2, 0]], 3);
        assert_eq!(array[[2, 1]], 6);
    }

    #[test]
    fn transpose_the_reshape() {
        // 2-D array:
        // 1 2 3
        // 4 5 6
        let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

        // tranpose the array to:
        // 1 4
        // 2 5
        // 3 6
        let array = array.transpose();

        // reshape the array to a 2x3 2-D array:
        // 1 4 2
        // 5 3 6
        let array = array.reshape([2, 3]);

        assert_eq!(array[[0, 0]], 1);
        assert_eq!(array[[0, 1]], 4);
        assert_eq!(array[[0, 2]], 2);
        assert_eq!(array[[1, 0]], 5);
        assert_eq!(array[[1, 1]], 3);
        assert_eq!(array[[1, 2]], 6);
    }

    #[test]
    fn flip() {
        // 2-D array:
        // 1 2 3
        // 4 5 6
        let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

        // flip axis = 0
        // 4 5 6
        // 1 2 3
        let array = array.flip(0);

        assert_eq!(array.iter().copied().collect::<Vec<usize>>(), vec![4, 5, 6, 1, 2, 3]);
    }

    #[test]
    fn d1() {
        let array = Array::init(vec![1, 2, 3, 4, 5, 6], [2, 3]);

        let array = array.reshape([6]);

        assert_eq!(array[[0]], 1)
    }
}
