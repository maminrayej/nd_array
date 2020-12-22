use std::ops::Index;

pub struct Matrix<T, const N: usize, const M: usize> {
    vec: Vec<T>,
}

impl<T, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn init(values: Vec<T>) -> Self {
        if values.len() != M * N {
            panic!(
                "Number of values is not equall to matrix size: {} != {}",
                values.len(),
                M * N
            );
        }

        Matrix { vec: values }
    }

    pub fn reshape<const P: usize, const Q: usize>(self) -> Matrix<T, P, Q> {
        if P * Q != M * N {
            panic!(
                "Size of new dimensions does not match the old one: {} != {}",
                P * Q,
                M * N
            );
        }

        Matrix { vec: self.vec }
    }
}

impl<T, const N: usize, const M: usize> Index<[usize; 2]> for Matrix<T, N, M> {
    type Output = T;

    fn index(&self, [i, j]: [usize; 2]) -> &Self::Output {
        let index = i * M + j;

        &self.vec[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_test() {
        let mat = Matrix::<usize, 3, 3>::init(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(mat[[1, 1]], 5);
    }

    #[test]
    fn matrix_reshape() {
        let mat = Matrix::<usize, 2, 2>::init(vec![1, 2, 3, 4]);

        let col_vec = mat.reshape::<4, 1>();

        assert_eq!(col_vec[[3, 0]], 4);
    }
}
