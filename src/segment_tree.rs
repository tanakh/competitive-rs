use crate::monoid::Monoid;
use std::{
    cmp::{max, min},
    ops::Bound,
    ops::RangeBounds,
};

/// Segment tree
#[derive(Debug)]
pub struct SegmentTree<T> {
    len: usize,
    v: Vec<T>,
}

impl<T: Clone + Monoid> SegmentTree<T> {
    /// O(n).
    /// Construct segment tree for given size.
    pub fn new(n: usize) -> Self {
        let s: &[T] = &[];
        Self::init(n, s)
    }

    /// O(n).
    /// Construct segment tree from slice.
    pub fn from_slice(s: &[impl Into<T> + Clone]) -> Self {
        Self::init(s.len(), s)
    }

    fn init(len: usize, s: &[impl Into<T> + Clone]) -> Self {
        let n = len.next_power_of_two();
        let mut v = vec![T::mempty(); n * 2 - 1];
        for i in 0..s.len() {
            v[n - 1 + i] = s[i].clone().into();
        }

        let mut l = n / 2;
        let mut ofs = n - 1 - l;

        while l > 0 {
            for i in 0..l {
                let ix = ofs + i;
                v[ix] = T::mappend(&v[ix * 2 + 1], &v[ix * 2 + 2]);
            }
            l /= 2;
            ofs -= l;
        }

        Self { len, v }
    }

    /// O(1).
    /// Length of sequence.
    pub fn len(&self) -> usize {
        self.len
    }

    /// O(log n).
    /// Set v to `i`-th element.
    /// `s[i] = v`
    pub fn set(&mut self, i: usize, v: impl Into<T>) {
        let n = (self.v.len() + 1) / 2;
        let mut cur = n - 1 + i;
        self.v[cur] = v.into();
        while cur > 0 {
            cur = (cur - 1) / 2;
            self.v[cur] = T::mappend(&self.v[cur * 2 + 1], &self.v[cur * 2 + 2]);
        }
    }

    /// O(log n).
    /// mappend v to `i`-th element
    /// `s[i] = mappend(s[i], v)`
    pub fn mappend(&mut self, i: usize, v: impl Into<T>) {
        self.set(i, T::mappend(&self.get(i), &v.into()));
    }

    /// O(1).
    /// Get i-th element
    /// Equals to `query(i, i + 1)`
    pub fn get(&self, i: usize) -> T {
        let n = (self.v.len() + 1) / 2;
        self.v[n - 1 + i].clone()
    }

    /// O(log n).
    /// Query for `range`.
    /// Returns `T::mconcat(&s[range])`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use competitive::monoid::Sum;
    /// # use competitive::segment_tree::SegmentTree;
    /// let mut st = SegmentTree::<Sum<i64>>::new(5);
    /// st.set(2, 3);
    /// assert_eq!(st.query(0..=2).0, 3);
    /// assert_eq!(st.query(0..2).0, 0);
    /// ```
    ///
    pub fn query(&self, range: impl RangeBounds<usize>) -> T {
        let l = match range.start_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => v + 1,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(v) => v + 1,
            Bound::Excluded(v) => *v,
            Bound::Unbounded => self.len,
        };

        assert!(l <= r);
        assert!(r <= self.len);

        let n = (self.v.len() + 1) / 2;
        let mut l = n + l;
        let mut r = n + r;

        let mut ret_l = T::mempty();
        let mut ret_r = T::mempty();
        while l < r {
            if l & 1 != 0 {
                ret_l = T::mappend(&ret_l, &self.v[l - 1]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                ret_r = T::mappend(&self.v[r - 1], &ret_r);
            }
            l /= 2;
            r /= 2;
        }

        T::mappend(&ret_l, &ret_r)
    }
}

#[test]
fn test() {
    use crate::monoid::Sum;

    {
        let st = SegmentTree::<Sum<i64>>::new(5);
        assert_eq!(st.v.iter().map(|r| r.0).collect::<Vec<_>>(), vec![0; 15]);
    }

    {
        let st = SegmentTree::<Sum<i64>>::from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(
            st.v.iter().map(|r| r.0).collect::<Vec<_>>(),
            vec![
                15, //
                10, 5, //
                3, 7, 5, 0, //
                1, 2, 3, 4, 5, 0, 0, 0
            ]
        );
    }

    let mut st = SegmentTree::<Sum<i64>>::new(5);
    st.set(2, 1);
    assert_eq!(st.query(0..5).0, 1);
    assert_eq!(st.query(0..2).0, 0);
    assert_eq!(st.query(3..5).0, 0);
    assert_eq!(st.query(2..3).0, 1);
    assert_eq!(st.query(2..2).0, 0);
    st.mappend(2, 2);
    assert_eq!(st.query(0..5).0, 3);
    assert_eq!(st.query(0..2).0, 0);
    assert_eq!(st.query(3..5).0, 0);
    assert_eq!(st.query(2..3).0, 3);
    assert_eq!(st.query(2..2).0, 0);
    st.set(0, 1);
    st.set(1, 2);
    st.set(3, 4);
    st.set(4, 5);
    assert_eq!(st.query(0..5).0, 15);
    assert_eq!(st.query(0..2).0, 3);
    assert_eq!(st.query(3..5).0, 9);
    assert_eq!(st.query(2..3).0, 3);
    assert_eq!(st.query(2..2).0, 0);
    assert_eq!(st.query(2..=2).0, 3);
    assert_eq!(st.query(..).0, 15);
    assert_eq!(st.query(1..).0, 14);
    assert_eq!(st.query(..3).0, 6);
    assert_eq!(st.query(..=3).0, 10);
}
