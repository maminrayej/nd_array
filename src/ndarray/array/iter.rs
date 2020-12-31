use crate::Array;

impl<'a, T: Clone, const D: usize> Array<'a, T, D> {
    pub fn iter(&self) -> Iter<'_, T, D> {
        Iter::init(self)
    }

    pub fn axes(&self) -> Axes<'_, D> {
        Axes::init(self.shape(), self.strides())
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

pub struct Axes<'a, const D: usize> {
    axis: usize,
    shape: &'a [usize; D],
    strides: &'a [usize; D],
}

impl<'a, const D: usize> Axes<'a, D> {
    pub fn init(shape: &'a [usize; D], strides: &'a [usize; D]) -> Self {
        Axes {
            axis: 0,
            shape,
            strides,
        }
    }
}

impl<'a, const D: usize> Iterator for Axes<'a, D> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let shape_stride = if self.axis < D {
            Some((self.shape[self.axis], self.strides[self.axis]))
        } else {
            None
        };

        self.axis += 1;

        shape_stride
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
