use std::cmp::min;
use std::ops::Add;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MaybeInf<T> {
    NotInf(T),
    Inf,
}

use MaybeInf::*;

impl<T> From<T> for MaybeInf<T> {
    fn from(v: T) -> Self {
        MaybeInf::NotInf(v)
    }
}

impl<T: Add<Output = T>> Add for MaybeInf<T> {
    type Output = MaybeInf<T>;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (NotInf(a), NotInf(b)) => NotInf(a + b),
            _ => Inf,
        }
    }
}

impl<T: Add<Output = T>> Add<T> for MaybeInf<T> {
    type Output = MaybeInf<T>;
    fn add(self, rhs: T) -> Self {
        match self {
            NotInf(a) => NotInf(a + rhs),
            _ => Inf,
        }
    }
}

impl<T: Clone + Ord> MaybeInf<T> {
    pub fn min_assign(&mut self, rhs: Self) {
        *self = min(self.clone(), rhs);
    }
}
