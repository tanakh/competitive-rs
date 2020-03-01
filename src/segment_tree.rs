use std::cmp::{max, min};

/// A trait of monoids
///
/// The class of monoids (types with an associative binary operation that has an identity). Instances should satisfy the following laws:
/// * `mappend(x, MEMPTY) = x`
/// * `mappend(MEMPTY, x) = x`
/// * `mappend(x, mappend(y, z)) = mappend(mappend(x, y), z)` (Semigroup law)
///
pub trait Monoid: Sized {
    /// Identity of `mappend`
    const MEMPTY: Self;

    /// An associative operation
    fn mappend(l: &Self, r: &Self) -> Self;

    /// Fold a slice using the monoid
    fn mconcat(xs: &[Self]) -> Self {
        xs.iter().fold(Self::MEMPTY, |a, b| Self::mappend(&a, b))
    }
}

/// Segment tree
#[derive(Debug)]
pub struct SegmentTree<T> {
    data: T,
    span: usize,
    l: Option<Box<SegmentTree<T>>>,
    r: Option<Box<SegmentTree<T>>>,
}

impl<T: Clone + Monoid> SegmentTree<T> {
    /// Construct segment tree for given size.
    pub fn new(n: usize) -> Self {
        Self::from_slice(&vec![T::MEMPTY; n])
    }

    /// Construct segment tree from slice.
    pub fn from_slice(s: &[T]) -> Self {
        if s.len() == 1 {
            SegmentTree {
                data: s[0].clone(),
                span: 1,
                l: None,
                r: None,
            }
        } else {
            let m = s.len() / 2;
            let l = Self::from_slice(&s[0..m]);
            let r = Self::from_slice(&s[m..]);
            Self {
                data: T::mappend(&l.data, &r.data),
                span: s.len(),
                l: Some(Box::new(l)),
                r: Some(Box::new(r)),
            }
        }
    }

    pub fn len(&self) -> usize {
        self.span
    }

    /// Update i-th element
    pub fn update(&mut self, i: usize, v: T) {
        if self.span == 1 {
            assert!(self.l.is_none());
            assert!(self.r.is_none());
            assert!(i == 0);

            self.data = v;
        } else {
            let m = self.span / 2;
            let l = self.l.as_mut().unwrap();
            let r = self.r.as_mut().unwrap();
            if i < m {
                l.update(i, v);
            } else {
                r.update(i - m, v);
            }

            self.data = T::mappend(&l.data, &r.data);
        }
    }

    /// Query for `[l, r)`.
    ///
    /// # Prerequiremens
    ///
    /// * `l <= r`
    /// * `r <= self.len()`
    ///
    /// # Returns
    ///
    /// `Monoid::mconcat(&s[l..r])`
    ///
    pub fn query(&self, l: usize, r: usize) -> T {
        assert!(l <= r);
        assert!(r <= self.span);

        if l == r {
            T::MEMPTY
        } else if r - l == self.span {
            self.data.clone()
        } else {
            let m = self.span / 2;
            let l_ref = self.l.as_ref().unwrap();
            let r_ref = self.r.as_ref().unwrap();

            T::mappend(
                &l_ref.query(min(l, m), min(r, m)),
                &r_ref.query(max(l, m) - m, max(r, m) - m),
            )
        }
    }
}
