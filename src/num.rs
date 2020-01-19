use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

// Fixme: use num trait

pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
    + Neg<Output = Output>
{
}

impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
        + Neg<Output = Output>
{
}

pub trait Num: PartialEq + NumOps + From<i32> {}

impl Num for i32 {}
impl Num for i64 {}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
pub struct Complex<T> {
    /// Real portion of the complex number
    pub re: T,
    /// Imaginary portion of the complex number
    pub im: T,
}

impl<T: Clone + Num> Complex<T> {
    /// Create a new Complex
    #[inline]
    pub fn new(re: T, im: T) -> Complex<T> {
        Complex { re: re, im: im }
    }

    #[inline]
    pub fn i() -> Complex<T> {
        Self::new(0.into(), 1.into())
    }

    #[inline]
    pub fn norm_sqr(&self) -> T {
        self.re.clone() * self.re.clone() + self.im.clone() * self.im.clone()
    }
}

impl<T: Clone + Num> Complex<T> {
    /// Returns the complex conjugate. i.e. `re - i im`
    #[inline]
    pub fn conj(&self) -> Complex<T> {
        Complex::new(self.re.clone(), -self.im.clone())
    }

    /// Returns `1/self`
    #[inline]
    pub fn inv(&self) -> Complex<T> {
        let norm_sqr = self.norm_sqr();
        Complex::new(
            self.re.clone() / norm_sqr.clone(),
            -self.im.clone() / norm_sqr,
        )
    }
}

/// Greatest common divisor
/// # Examples
///
/// ```
/// # use competitive::num::*;
/// assert_eq!(gcd(57, 3), 3)
/// ```
pub fn gcd<T: Num + Copy>(a: T, b: T) -> T {
    if b == 0.into() {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Least common multiple
/// # Examples
///
/// ```
/// # use competitive::num::*;
/// assert_eq!(lcm(12, 8), 24)
/// ```
pub fn lcm<T: Num + Copy>(a: T, b: T) -> T {
    let g = gcd(a, b);
    a / g * b
}

const M: i64 = 1000000007;

#[derive(Debug, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct GF(i64);

impl ::std::fmt::Display for GF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl GF {
    pub fn new(v: i64) -> GF {
        GF(v % M)
    }

    pub fn pow(self, mut r: i64) -> GF {
        let mut k = self;
        let mut ret = 1.into();

        while r > 0 {
            if r % 2 != 0 {
                ret = ret * k;
            }
            r /= 2;
            k = k * k;
        }

        ret
    }

    // This requires M is prime
    pub fn recip(self) -> GF {
        self.pow(M - 2)
    }
}

impl<T: Into<GF>> Add<T> for GF {
    type Output = GF;
    fn add(self, rhs: T) -> Self::Output {
        GF::new(self.0 + rhs.into().0)
    }
}
impl<T: Into<GF>> AddAssign<T> for GF {
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T: Into<GF>> Sub<T> for GF {
    type Output = GF;
    fn sub(self, rhs: T) -> Self::Output {
        GF::new(self.0 - rhs.into().0 + M)
    }
}
impl<T: Into<GF>> SubAssign<T> for GF {
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T: Into<GF>> Mul<T> for GF {
    type Output = GF;
    fn mul(self, rhs: T) -> Self::Output {
        GF::new(self.0 * rhs.into().0)
    }
}
impl<T: Into<GF>> MulAssign<T> for GF {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T: Into<GF>> Div<T> for GF {
    type Output = GF;
    fn div(self, rhs: T) -> Self::Output {
        self * rhs.into().recip()
    }
}
impl<T: Into<GF>> DivAssign<T> for GF {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl Neg for GF {
    type Output = GF;
    fn neg(self) -> Self::Output {
        GF(0) - self
    }
}

impl<T: ::std::convert::Into<i64>> ::std::convert::From<T> for GF {
    fn from(v: T) -> Self {
        GF::new(v.into())
    }
}

#[test]
fn test_modular() {
    let x: GF = 12345678.into();
    let y: GF = 87654321.into();
    assert_eq!(y * x * x.recip(), y);
}
