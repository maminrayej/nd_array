use std::borrow::Cow;

use crate::Array;

impl<'a, T: Clone, const D: usize> Array<'a, T, D> {
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
            idx_maps,
        }
    }

    pub fn flip(&'a self, axis: usize) -> Array<'a, T, D> {
        if axis >= D {
            panic!("Axis out of bounds")
        }

        let mut idx_maps = self.idx_maps.clone();

        let idx_map = &mut idx_maps[axis];

        idx_map.append_b((self.shape[axis] - 1) as isize);
        idx_map.m *= -1;

        Array {
            vec: Cow::from(&*self.vec),
            shape: self.shape.clone(),
            strides: self.strides.clone(),
            idx_maps,
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
            idx_maps,
        }
    }

    pub fn reshape<const S: usize>(&self, shape: [usize; S]) -> Array<'a, T, S> {
        // TODO: Check wether cloning is necessary

        let vec = self.flat().cloned().collect();

        Array::init(vec, shape)
    }

    pub fn flatten(&self) -> Array<'a, T, 1> {
        let vec = self.flat().cloned().collect();

        Array::init(vec, [self.vec.len()])
    }

    pub fn ravel(&self) -> Array<'a, T, 1> {
        self.reshape([self.vec.len()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(
            array.flat().copied().collect::<Vec<usize>>(),
            vec![4, 5, 6, 1, 2, 3]
        );
    }
}
