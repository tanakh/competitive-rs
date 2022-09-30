use std::iter::{Product, Sum};

pub trait IteratorExt: Iterator {
    /// monomorphic version of `Iterator::sum`
    fn sum_(self) -> Self::Item
    where
        Self: Sized,
        Self::Item: Ord,
        Self::Item: Sum<Self::Item>,
    {
        self.sum()
    }

    /// monomorphic version of `Iterator::product`
    fn product_(self) -> Self::Item
    where
        Self: Sized,
        Self::Item: Ord,
        Self::Item: Product<Self::Item>,
    {
        self.product()
    }
}

impl<T: Iterator> IteratorExt for T {}
