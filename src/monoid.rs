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
