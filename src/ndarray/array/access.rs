use std::{
    borrow::Cow,
    ops::{Index, Range},
};

use crate::Array;

impl<'a, T: Clone, const D: usize> Array<'a, T, D> {
    pub fn slice(&'a self, ranges: &[Range<usize>; D]) -> Array<'a, T, D> {
        let mut shape = self.shape.clone();
        let strides = self.strides.clone();
        let mut idx_maps = self.idx_maps.clone();

        ranges.iter().enumerate().for_each(|(axis, range)| {
            if range.end > self.shape[axis] {
                panic!(
                    "Range: [{},{}) is out of bounds for axis: {}",
                    range.start, range.end, axis
                )
            }
        });

        for axis in 0..D {
            idx_maps[axis].append_b((ranges[axis].start) as isize);
            shape[axis] = ranges[axis].end - ranges[axis].start;
        }

        Array {
            vec: Cow::from(&*self.vec),
            shape,
            strides,
            idx_maps,
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
    fn slicing() {
        // 2-D array:
        // 1   2  3  4
        // 5   6  7  8
        // 9  10 11 12
        // 13 14 15 16
        let array = Array::init(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            [4, 4],
        );

        // flip the array across axis=0
        // 13 14 15 16
        // 9  10 11 12
        // 5  6  7  8
        // 1  2  3  4
        let flipped = array.flip(0);

        // slice the center of the array
        // 10 11
        // 6  7
        let slice = flipped.slice(&[1..3, 1..3]);

        // 11 10
        // 7  6
        let flip_of_slice = slice.flip(1);

        assert_eq!(
            flip_of_slice.flat().copied().collect::<Vec<usize>>(),
            vec![11, 10, 7, 6]
        );
    }
}
