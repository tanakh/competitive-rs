use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

const M: i64 = 1000000007;

#[derive(Debug, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct GF(i64);

impl std::fmt::Display for GF {
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

impl<T: std::convert::Into<i64>> std::convert::From<T> for GF {
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
