use num::{One, Zero};
use std::ops::{Add, Mul};

/// A trait of monoids
///
/// The class of monoids (types with an associative binary operation that has an identity). Instances should satisfy the following laws:
/// * `mappend(x, MEMPTY) = x`
/// * `mappend(MEMPTY, x) = x`
/// * `mappend(x, mappend(y, z)) = mappend(mappend(x, y), z)` (Semigroup law)
///
pub trait Monoid: Sized {
    /// Identity of `mappend`
    fn mempty() -> Self;

    /// An associative operation
    fn mappend(l: &Self, r: &Self) -> Self;

    /// Fold a slice using the monoid
    fn mconcat(xs: &[Self]) -> Self {
        xs.iter().fold(Self::mempty(), |a, b| Self::mappend(&a, b))
    }
}

pub struct Sum<T>(pub T);

impl<T: Copy + Zero + Add<Output = T>> Monoid for Sum<T> {
    fn mempty() -> Self {
        Self(T::zero())
    }

    fn mappend(l: &Self, r: &Self) -> Self {
        Self(l.0 + r.0)
    }
}

impl<T> From<T> for Sum<T> {
    fn from(v: T) -> Self {
        Sum(v)
    }
}

// impl<T> From<Sum<T>> for T {
//     fn from(v: Sum<T>) -> Self {
//         v.0
//     }
// }

pub struct Product<T>(pub T);

impl<T: Copy + One + Mul<Output = T>> Monoid for Product<T> {
    fn mempty() -> Self {
        Self(T::one())
    }

    fn mappend(l: &Self, r: &Self) -> Self {
        Self(l.0 * r.0)
    }
}

impl<T> From<T> for Product<T> {
    fn from(v: T) -> Self {
        Product(v)
    }
}

// impl<T> From<Product<T>> for T {
//     fn from(v: Product<T>) -> Self {
//         v.0
//     }
// }
