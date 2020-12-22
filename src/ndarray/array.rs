use std::ops::Index;

pub struct Array<T, const D: usize> {
    vec: Vec<T>,
    dim: [usize; D],
}

impl<T, const D: usize> Array<T, D> {
    pub fn init(vec: Vec<T>, dim: [usize; D]) -> Self {
        let elem_count = dim.iter().fold(1, |acc, v| acc * v);

        if elem_count != vec.len() {
            panic!(
                "Number of elements in vec is not equal to dimension specification: {} != {}",
                vec.len(),
                elem_count
            );
        }

        Array { vec, dim }
    }

    pub fn reshape<const S: usize>(self, dim: [usize; S]) -> Array<T, S> {
        Array::init(self.vec, dim)
    }
}

impl<T, const D: usize> Index<[usize; D]> for Array<T, D> {
    type Output = T;

    fn index(&self, indices: [usize; D]) -> &Self::Output {
        let index: usize = (0..D)
            .map(|axis| indices[axis] * self.dim[axis + 1..].iter().fold(1, |acc, v| acc * v))
            .sum();

        &self.vec[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let array = Array::init(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], [3, 3]);

        assert_eq!(array[[1, 1]], 5);
    }
}
