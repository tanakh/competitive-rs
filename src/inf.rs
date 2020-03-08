use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum MaybeInf<T> {
    NegInf,
    NonInf(T),
    Inf,
}

use MaybeInf::*;

impl<T> MaybeInf<T> {
    pub fn unwrap(self) -> T {
        match self {
            NegInf => panic!(),
            NonInf(a) => a,
            Inf => panic!(),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            NonInf(a) => a,
            _ => default,
        }
    }

    pub fn option(self) -> Option<T> {
        match self {
            NonInf(a) => Some(a),
            _ => None,
        }
    }
}

impl<T> From<T> for MaybeInf<T> {
    fn from(v: T) -> Self {
        NonInf(v)
    }
}

impl<T: Add<Output = T>> Add for MaybeInf<T> {
    type Output = MaybeInf<T>;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (NegInf, NegInf) => NegInf,
            (NegInf, NonInf(_)) => NegInf,
            (NegInf, Inf) => panic!("add -inf + inf"),
            (NonInf(_), NegInf) => NegInf,
            (NonInf(a), NonInf(b)) => NonInf(a + b),
            (NonInf(_), Inf) => Inf,
            (Inf, NegInf) => panic!("add inf + (-inf)"),
            (Inf, NonInf(_)) => Inf,
            (Inf, Inf) => Inf,
        }
    }
}

impl<T: Add<Output = T>> Add<T> for MaybeInf<T> {
    type Output = MaybeInf<T>;
    fn add(self, rhs: T) -> Self {
        match self {
            NegInf => NegInf,
            NonInf(a) => NonInf(a + rhs),
            Inf => Inf,
        }
    }
}

impl<T: Add<Output = T> + Clone> AddAssign for MaybeInf<T> {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<T: Add<Output = T> + Clone> AddAssign<T> for MaybeInf<T> {
    fn add_assign(&mut self, other: T) {
        *self = self.clone() + other;
    }
}

impl<T: Sub<Output = T>> Sub for MaybeInf<T> {
    type Output = MaybeInf<T>;
    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (NegInf, NegInf) => panic!("sub -inf - (-inf)"),
            (NegInf, NonInf(_)) => NegInf,
            (NegInf, Inf) => NegInf,
            (NonInf(_), NegInf) => Inf,
            (NonInf(a), NonInf(b)) => NonInf(a - b),
            (NonInf(_), Inf) => NegInf,
            (Inf, NegInf) => Inf,
            (Inf, NonInf(_)) => Inf,
            (Inf, Inf) => panic!("sub inf - inf"),
        }
    }
}

impl<T: Sub<Output = T>> Sub<T> for MaybeInf<T> {
    type Output = MaybeInf<T>;
    fn sub(self, rhs: T) -> Self {
        match self {
            NegInf => NegInf,
            NonInf(a) => NonInf(a - rhs),
            Inf => Inf,
        }
    }
}

impl<T: Sub<Output = T> + Clone> SubAssign for MaybeInf<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl<T: Sub<Output = T> + Clone> SubAssign<T> for MaybeInf<T> {
    fn sub_assign(&mut self, other: T) {
        *self = self.clone() - other;
    }
}

impl<T: Neg<Output = T>> Neg for MaybeInf<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            NegInf => Inf,
            NonInf(a) => NonInf(-a),
            Inf => NegInf,
        }
    }
}

impl<T: Mul<Output = T> + PartialOrd + From<u8>> Mul for MaybeInf<T> {
    type Output = MaybeInf<T>;
    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (NegInf, NegInf) => Inf,
            (NegInf, NonInf(b)) => {
                if b > 0_u8.into() {
                    NegInf
                } else if b == 0_u8.into() {
                    panic!("-inf * 0")
                } else {
                    Inf
                }
            }
            (NegInf, Inf) => NegInf,
            (NonInf(a), NegInf) => NegInf::<T> * NonInf(a),
            (NonInf(a), NonInf(b)) => NonInf(a * b),
            (NonInf(a), Inf) => {
                if a > 0_u8.into() {
                    Inf
                } else if a == 0_u8.into() {
                    panic!("inf * 0");
                } else {
                    NegInf
                }
            }
            (Inf, NegInf) => NegInf,
            (Inf, NonInf(b)) => NonInf(b) * Inf,
            (Inf, Inf) => Inf,
        }
    }
}

impl<T: Mul<Output = T>> Mul<T> for MaybeInf<T> {
    type Output = MaybeInf<T>;
    fn mul(self, rhs: T) -> Self {
        match self {
            NegInf => NegInf,
            NonInf(a) => NonInf(a * rhs),
            Inf => Inf,
        }
    }
}

impl<T: Mul<Output = T> + Clone + PartialOrd + From<u8>> MulAssign for MaybeInf<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<T: Mul<Output = T> + Clone> MulAssign<T> for MaybeInf<T> {
    fn mul_assign(&mut self, other: T) {
        *self = self.clone() * other;
    }
}

#[test]
fn test_inf() {
    use std::cmp::{max, min};

    let mut t: MaybeInf<i32> = Inf;
    t = min(t, NonInf(123));
    assert_eq!(t, NonInf(123));
    t = min(t, Inf);
    assert_eq!(t, NonInf(123));
    t = min(t, NegInf);
    assert_eq!(t, NegInf);
    t = max(t, NonInf(123));
    assert_eq!(t, NonInf(123));
    t = max(t, Inf);
    assert_eq!(t, Inf);

    t = -Inf;
    assert_eq!(t, NegInf);

    assert_eq!(NonInf(123_i32) + 456, NonInf(579));

    t = 0.into();
    t += 100;
    assert_eq!(t, NonInf(100));
    t += NonInf(100);
    assert_eq!(t, NonInf(200));
    t += Inf;
    assert_eq!(t, Inf);
    t = 0.into();
    t += NegInf;
    assert_eq!(t, NegInf);

    t = 0.into();
    t -= 100;
    assert_eq!(t, NonInf(-100));
    t -= NonInf(100);
    assert_eq!(t, NonInf(-200));
    t -= Inf;
    assert_eq!(t, NegInf);
    t = 0.into();
    t -= NegInf;
    assert_eq!(t, Inf);

    t = 1.into();
    t *= 100;
    assert_eq!(t, NonInf(100));
    t *= NonInf(100);
    assert_eq!(t, NonInf(10000));
    t *= Inf;
    assert_eq!(t, Inf);
    t = 1.into();
    t *= NegInf;
    assert_eq!(t, NegInf);
}
