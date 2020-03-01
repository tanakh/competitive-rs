use crate::monoid::Monoid;
use num::{PrimInt, Unsigned};
use std::ops::Index;

#[derive(Clone)]
pub struct SmallBitSet<T>(pub T);

impl<T: PrimInt + Unsigned> SmallBitSet<T> {
    pub fn new() -> Self {
        Self(T::zero())
    }

    pub fn singleton(i: usize) -> Self {
        Self(T::one() << i)
    }

    pub fn count(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn set(&mut self, i: usize, value: bool) {
        self.0 = self.0 & !(T::one() << i) | ((if value { T::one() } else { T::zero() }) << i);
    }
}

impl<T: PrimInt + Unsigned> Index<usize> for SmallBitSet<T> {
    type Output = bool;
    fn index(&self, index: usize) -> &bool {
        if self.0 & (T::one() << index) != T::zero() {
            &true
        } else {
            &false
        }
    }
}

impl<T: PrimInt + Unsigned> Monoid for SmallBitSet<T> {
    fn mempty() -> Self {
        SmallBitSet(T::zero())
    }

    fn mappend(l: &Self, r: &Self) -> Self {
        Self(l.0 | r.0)
    }
}

/// Returns power of bitset (n <= 31)
pub fn power_bitset(n: usize) -> impl Iterator<Item = SmallBitSet<u32>> {
    assert!(n <= 31);
    PowerBitSetIter {
        cur: 0,
        n: n as u32,
    }
}

struct PowerBitSetIter {
    cur: u32,
    n: u32,
}

impl Iterator for PowerBitSetIter {
    type Item = SmallBitSet<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < (1 << self.n) {
            let ret = SmallBitSet(self.cur);
            self.cur += 1;
            Some(ret)
        } else {
            None
        }
    }
}
