use std::cmp::min;
use std::ops::Add;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Inf<T> {
    NotInf(T),
    Inf,
}

use self::Inf::*;

impl<T> From<T> for Inf<T> {
    fn from(v: T) -> Self {
        Inf::NotInf(v)
    }
}

impl<T: Add<Output = T>> Add for Inf<T> {
    type Output = Inf<T>;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (NotInf(a), NotInf(b)) => NotInf(a + b),
            _ => Inf,
        }
    }
}

impl<T: Add<Output = T>> Add<T> for Inf<T> {
    type Output = Inf<T>;
    fn add(self, rhs: T) -> Self {
        match self {
            NotInf(a) => NotInf(a + rhs),
            _ => Inf,
        }
    }
}

impl<T: Clone + Ord> Inf<T> {
    pub fn min_assign(&mut self, rhs: Self) {
        *self = min(self.clone(), rhs);
    }
}
