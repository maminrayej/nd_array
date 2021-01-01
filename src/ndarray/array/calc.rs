use crate::Array;

impl<'a, T: Clone + Ord, const D: usize> Array<'a, T, D> {
    pub fn max(&self) -> Option<T> {
        self.flat().max().cloned()
    }

    pub fn arg_max(&self) -> Vec<usize> {
        let mut positions = vec![];

        if let Some(max) = self.max() {
            for (index, value) in self.flat().enumerate() {
                if value == &max {
                    positions.push(index)
                }
            }
        }

        positions
    }

    pub fn max_across(&self, axis: usize) -> Vec<Option<T>> {
        self.axis_view(axis).map(|view| view.max()).collect()
    }

    pub fn arg_max_across(&self, axis: usize) -> Vec<Option<usize>> {
        self.axis_view(axis)
            .map(|view| view.arg_max().get(0).copied())
            .collect()
    }

    pub fn min(&self) -> Option<T> {
        self.flat().min().cloned()
    }

    pub fn arg_min(&self) -> Vec<usize> {
        let mut positions = vec![];

        if let Some(min) = self.min() {
            for (index, value) in self.flat().enumerate() {
                if value == &min {
                    positions.push(index)
                }
            }
        }

        positions
    }

    pub fn min_across(&self, axis: usize) -> Vec<Option<T>> {
        self.axis_view(axis).map(|view| view.min()).collect()
    }

    pub fn arg_min_across(&self, axis: usize) -> Vec<Option<usize>> {
        self.axis_view(axis)
            .map(|view| view.arg_min().get(0).copied())
            .collect()
    }
}
