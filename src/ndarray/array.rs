use std::ops::Index;

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

    fn set_b(&mut self, b: isize) {
        self.b = b
    }

    fn set_m(&mut self, m: isize) {
        self.m = m
    }

    fn m(&self) -> isize {
        self.m
    }

    fn b(&self) -> isize {
        self.b
    }
}

pub struct Array<T, const D: usize> {
    vec: Vec<T>,
    shape: [usize; D],
    strides: [usize; D],
    idx_map: [IdxMap; D],
}

impl<T, const D: usize> Array<T, D> {
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
            vec,
            shape,
            strides,
            idx_map: [IdxMap::init(); D],
        }
    }

    pub fn transpose(mut self) -> Array<T, D> {
        self.shape.reverse();
        self.strides.reverse();
        self.idx_map.reverse();

        self
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
                acc + self.idx_map[axis].map(*axis_index) * self.strides[axis]
            });

        self.vec.get(index)
    }

    pub fn iter(&self) -> Iter<'_, T, D> {
        Iter::init(self)
    }

    pub fn flip(mut self, axis: usize) -> Array<T, D> {
        if !(0..D).contains(&axis) {
            panic!("Axis out of bounds")
        }

        let idx_map = &mut self.idx_map[axis];

        if idx_map.m() == -1 {
            idx_map.m = 1;
            idx_map.b = idx_map.b - (self.shape[axis] - 1) as isize
        } else {
            idx_map.m = -1;
            idx_map.b = idx_map.b + (self.shape[axis] - 1) as isize
        }

        self
    }
}

impl<T: Clone, const D: usize> Array<T, D> {
    pub fn reshape<const S: usize>(self, shape: [usize; S]) -> Array<T, S> {
        // TODO: Check if c-contigous

        let vec = self.iter().cloned().collect();

        Array::init(vec, shape)
    }
}

impl<T, const D: usize> Index<[usize; D]> for Array<T, D> {
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
                acc + self.idx_map[axis].map(*axis_index) * self.strides[axis]
            });

        &self.vec[index]
    }
}

pub struct Iter<'a, T, const D: usize> {
    array: &'a Array<T, D>,
    indices: [usize; D],
}

impl<'a, T, const D: usize> Iter<'a, T, D> {
    fn init(array: &'a Array<T, D>) -> Self {
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

impl<'a, T, const D: usize> Iterator for Iter<'a, T, D> {
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
}
