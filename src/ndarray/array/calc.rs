use crate::Array;

impl<'a, T: Clone + PartialOrd, const D: usize> Array<'a, T, D> {
    pub fn max(&self, axis: usize) -> &T {
        todo!()
    }
}