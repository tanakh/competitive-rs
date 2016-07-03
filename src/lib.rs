pub mod io {
    use std::io;

    pub fn read_line() -> String {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned()
    }

    pub trait Read {
        fn read(s: &str) -> Self;
    }

    macro_rules! def_read {
        ($t: ty) => {
            impl Read for $t {
                fn read(s: &str) -> $t {
                    s.parse().unwrap()
                }
            }
        }
    }

    def_read!(i32);
    def_read!(i64);
    def_read!(isize);
    def_read!(usize);
    def_read!(f64);

    impl<T: Read> Read for Vec<T> {
        fn read(s: &str) -> Self {
            s.split_whitespace().map(|w| T::read(w)).collect()
        }
    }

    macro_rules! def_read_tuple {
        ($($t: ident),*) => {
            impl<$($t : Read + Default),*> Read for ($($t),*) {
                fn read(s: &str) -> Self {
                    let ws = s.split_whitespace().collect::<Vec<_>>();
                    let mut i = 0;
                    ( $({i += 1; $t::read(ws[i - 1])}),* )
                }
            }
        };
    }

    def_read_tuple!(T0, T1);
    def_read_tuple!(T0, T1, T2);
    def_read_tuple!(T0, T1, T2, T3);
    def_read_tuple!(T0, T1, T2, T3, T4);

    pub fn readln<T: Read>() -> T {
        T::read(&read_line())
    }
}

pub mod modulo {
    use std::ops;
    use std::fmt;

    const M: i64 = 1000000007;

    #[derive(Debug, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
    pub struct Int(i64);

    impl fmt::Display for Int {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Int {
        pub fn new(v: i64) -> Int {
            Int(v % M)
        }

        pub fn pow(self, mut r: i64) -> Int {
            let mut k = self;
            let mut ret = Int::new(1);

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
        pub fn recip(self) -> Int {
            self.pow(M - 2)
        }
    }

    impl ops::Add for Int {
        type Output = Int;
        fn add(self, rhs: Int) -> Int {
            Int::new(self.0 + rhs.0)
        }
    }
    impl ops::Sub for Int {
        type Output = Int;
        fn sub(self, rhs: Int) -> Int {
            Int::new(self.0 - rhs.0 + M)
        }
    }
    impl ops::Mul for Int {
        type Output = Int;
        fn mul(self, rhs: Int) -> Int {
            Int::new(self.0 * rhs.0)
        }
    }
    impl ops::Div for Int {
        type Output = Int;
        fn div(self, rhs: Int) -> Int {
            self * rhs.recip()
        }
    }

    // impl ops::AddAssign for Int {
    //     fn add_assign(&mut self, rhs: Int) {
    //         *self = *self + rhs;
    //     }
    // }
    // impl ops::SubAssign for Int {
    //     fn sub_assign(&mut self, rhs: Int) {
    //         *self = *self - rhs;
    //     }
    // }
    // impl ops::MulAssign for Int {
    //     fn mul_assign(&mut self, rhs: Int) {
    //         *self = *self * rhs;
    //     }
    // }
    // impl ops::DivAssign for Int {
    //     fn div_assign(&mut self, rhs: Int) {
    //         *self = *self / rhs;
    //     }
    // }

    impl ops::Neg for Int {
        type Output = Int;
        fn neg(self) -> Int {
            Int(0) - self
        }
    }

    impl ::io::Read for Int {
        fn read(s: &str) -> Int {
            Int::new(i64::read(s))
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
