use std::{
    cmp::{max, min},
    ops::{Range, Sub},
};

pub trait RangeExt: Sized {
    // FIXME: remove this when Range::is_empty() become stable.
    fn empty(&self) -> bool;
    fn contains(&self, rhs: Self) -> bool;

    fn intersection(&self, rhs: Self) -> Option<Self>;
    fn union(&self, rhs: Self) -> Option<Self>;
    fn difference(&self, rhs: Self) -> Vec<Self>;
}

impl<Idx: Eq + Ord + Sub + Copy> RangeExt for Range<Idx> {
    fn empty(&self) -> bool {
        self.start == self.end
    }

    fn contains(&self, rhs: Self) -> bool {
        self.start <= rhs.start && rhs.end <= self.end
    }

    fn intersection(&self, rhs: Self) -> Option<Self> {
        let start = max(self.start, rhs.start);
        let end = min(self.end, rhs.end);
        if start < end {
            Some(start..end)
        } else {
            None
        }
    }

    fn union(&self, rhs: Self) -> Option<Self> {
        if self.intersection(rhs.clone()).is_none() {
            None
        } else {
            let start = min(self.start, rhs.start);
            let end = max(self.end, rhs.end);
            Some(start..end)
        }
    }

    fn difference(&self, rhs: Self) -> Vec<Self> {
        if self.intersection(rhs.clone()).is_none() {
            return vec![self.clone()];
        }

        let mut ret = Vec::with_capacity(2);
        if self.start < rhs.start {
            ret.push(self.start..rhs.start);
        }
        if rhs.end < self.end {
            ret.push(rhs.end..self.end);
        }
        ret
    }
}

#[test]
fn test() {
    assert_eq!((0..10).intersection(5..15), Some(5..10));
    assert_eq!((0..10).intersection(15..25), None);

    assert_eq!((0..10).union(5..15), Some(0..15));
    assert_eq!((0..10).union(15..25), None);

    assert_eq!((0..10).difference(15..25), vec![0..10]);
    assert_eq!((0..10).difference(10..25), vec![0..10]);
    assert_eq!((0..10).difference(5..25), vec![0..5]);
    assert_eq!((0..10).difference(0..25), vec![]);
    assert_eq!((0..10).difference(0..10), vec![]);
    assert_eq!((0..10).difference(0..5), vec![5..10]);
    assert_eq!((0..10).difference(2..5), vec![0..2, 5..10]);
    assert_eq!((5..10).difference(0..7), vec![7..10]);
    assert_eq!((5..10).difference(0..15), vec![]);
}
