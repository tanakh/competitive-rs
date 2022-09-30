use std::convert::{From, Into, TryInto};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Finite field of prime `P`
///
/// `P` must be a prime number.
///
/// ```
/// use competitive::prelude::*;
///
/// type GF = competitive::gf::GF<1000000007>;
///
/// let t = GF::new(2);
/// assert_eq!(t.pow(100).as_u64(), 976371285);
/// ```
///
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct GF<const P: u64>(pub u64);

impl<const P: u64> Display for GF<P> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const P: u64> GF<P> {
    pub fn new<T: TryInto<i64>>(v: T) -> Self {
        Self(v.try_into().ok().unwrap().rem_euclid(P as i64) as u64)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn pow(self, mut r: u64) -> Self {
        let mut k = self;
        let mut ret = GF::<P>::new(1);
        while r > 0 {
            if r % 2 != 0 {
                ret = ret * k;
            }
            r /= 2;
            k = k * k;
        }
        ret
    }

    // This requires P is prime
    pub fn recip(self) -> Self {
        self.pow(P - 2)
    }
}

impl<T: Into<GF<P>>, const P: u64> Add<T> for GF<P> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self::new(self.0 + rhs.into().0)
    }
}

impl<T: Into<GF<P>>, const P: u64> AddAssign<T> for GF<P> {
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T: Into<GF<P>>, const P: u64> Sub<T> for GF<P> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self::new(self.0 + P - rhs.into().0)
    }
}

impl<T: Into<GF<P>>, const P: u64> SubAssign<T> for GF<P> {
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T: Into<GF<P>>, const P: u64> Mul<T> for GF<P> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.0 * rhs.into().0)
    }
}

impl<T: Into<GF<P>>, const P: u64> MulAssign<T> for GF<P> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T: Into<GF<P>>, const P: u64> Div<T> for GF<P> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self * rhs.into().recip()
    }
}

impl<T: Into<GF<P>>, const P: u64> DivAssign<T> for GF<P> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

// FIXME: Currently, rustc does not allow partial orphan instance
// Use trait when it will be stabilized
macro_rules! def_ops {
    ($ty:ty) => {
        impl<const P: u64> Add<GF<P>> for $ty {
            type Output = GF<P>;
            fn add(self, rhs: GF<P>) -> Self::Output {
                GF::<P>::new(self) + rhs
            }
        }
        impl<const P: u64> Sub<GF<P>> for $ty {
            type Output = GF<P>;
            fn sub(self, rhs: GF<P>) -> Self::Output {
                GF::<P>::new(self) - rhs
            }
        }
        impl<const P: u64> Mul<GF<P>> for $ty {
            type Output = GF<P>;
            fn mul(self, rhs: GF<P>) -> Self::Output {
                GF::<P>::new(self) * rhs
            }
        }
        impl<const P: u64> Div<GF<P>> for $ty {
            type Output = GF<P>;
            fn div(self, rhs: GF<P>) -> Self::Output {
                GF::<P>::new(self) / rhs
            }
        }
    };
}

def_ops!(i8);
def_ops!(i16);
def_ops!(i32);
def_ops!(i64);
def_ops!(isize);

def_ops!(u8);
def_ops!(u16);
def_ops!(u32);
def_ops!(u64);
def_ops!(usize);

impl<const P: u64> Neg for GF<P> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(0) - self
    }
}

impl<T: TryInto<i64>, const P: u64> From<T> for GF<P> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_gf() {
        type GF = super::GF<1000000007>;

        // operator test
        let _ = GF::new(0);
        let x: GF = 0.into();
        let x = -x;

        let x = 1 + x;
        let x = x + 1;
        let mut x = x + x;
        x += 1;

        let x = 1 - x;
        let x = x - 1;
        let mut x = x - x;
        x -= 1;

        let x = 1 * x;
        let x = x * 1;
        let mut x = x * x;
        x *= 1;

        let x = 1 / x;
        let x = x / 1;
        let mut x = x / x;
        x /= 1;

        // basic tests
        let x: GF = 12345678.into();
        let y: GF = 87654321.into();
        assert_eq!(y * x * x.recip(), y);

        assert_eq!(GF::new(2).pow(50).0, (1 << 50) % 1000000007)
    }
}
