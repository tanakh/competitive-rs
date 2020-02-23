/// Define a finite field for given prime
#[macro_export]
macro_rules! def_gf {
    ($p:expr) => {
        mod gf {
            use std::convert::{From, Into, TryInto};
            use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

            #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
            pub struct GF(pub u64);

            pub const P: u64 = $p;

            impl std::fmt::Display for GF {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }

            impl $crate::util::Echo for GF {
                fn echo(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                    write!(w, "{}", self.0)
                }
            }

            impl GF {
                pub fn new<T: TryInto<i64>>(v: T) -> GF {
                    GF(v.try_into().ok().unwrap().rem_euclid(P as i64) as u64)
                }

                pub fn pow(self, mut r: u64) -> GF {
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
                    self.pow(P - 2)
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
                    GF::new(self.0 + P - rhs.into().0)
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

            macro_rules! def_ops {
                ($ty:ty) => {
                    impl Add<GF> for $ty {
                        type Output = GF;
                        fn add(self, rhs: GF) -> Self::Output {
                            GF::new(self) + rhs
                        }
                    }
                    impl Sub<GF> for $ty {
                        type Output = GF;
                        fn sub(self, rhs: GF) -> Self::Output {
                            GF::new(self) - rhs
                        }
                    }
                    impl Mul<GF> for $ty {
                        type Output = GF;
                        fn mul(self, rhs: GF) -> Self::Output {
                            GF::new(self) * rhs
                        }
                    }
                    impl Div<GF> for $ty {
                        type Output = GF;
                        fn div(self, rhs: GF) -> Self::Output {
                            GF::new(self) / rhs
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

            impl Neg for GF {
                type Output = GF;
                fn neg(self) -> Self::Output {
                    GF::new(0) - self
                }
            }

            impl<T: TryInto<i64>> From<T> for GF {
                fn from(v: T) -> Self {
                    GF::new(v)
                }
            }
        }

        use gf::GF;
    };
}

#[test]
fn test_gf() {
    def_gf!(1000000007);

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

    assert_eq!(GF::new(2).pow(50).0, (1 << 50) % gf::P)
}
