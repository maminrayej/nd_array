use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    Range, RangeBounds,
};

pub fn bounded_range_of<R: RangeBounds<usize>>(upper_bound: usize, range: &R) -> Range<usize> {
    match (range.start_bound(), range.end_bound()) {
        (Included(&start), Included(&end)) => start..end.checked_add(1).unwrap_or(upper_bound),
        (Included(&start), Excluded(&end)) => start..end,
        (Included(&start), Unbounded) => start..upper_bound,
        (Excluded(&start), Included(&end)) => start + 1..end.checked_add(1).unwrap_or(upper_bound),
        (Excluded(&start), Excluded(&end)) => start + 1..end,
        (Excluded(&start), Unbounded) => start + 1..upper_bound,
        (Unbounded, Included(&end)) => 0..end + 1,
        (Unbounded, Excluded(&end)) => 0..end,
        (Unbounded, Unbounded) => 0..upper_bound,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_range() {
        assert_eq!(bounded_range_of(10, &(..3)), 0..3);
        assert_eq!(bounded_range_of(10, &(..)), 0..10);
        assert_eq!(bounded_range_of(10, &(1..)), 1..10);
        assert_eq!(bounded_range_of(10, &(1..3)), 1..3);
        assert_eq!(bounded_range_of(10, &(1..=3)), 1..4);
    }
}
