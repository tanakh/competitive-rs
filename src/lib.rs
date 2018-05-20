pub mod io {
    use std;
    use std::str::FromStr;

    pub struct Scanner<'a> {
        iter: std::str::SplitWhitespace<'a>,
    }

    impl<'a> Scanner<'a> {
        pub fn new(s: &'a str) -> Scanner<'a> {
            Scanner {
                iter: s.split_whitespace(),
            }
        }

        pub fn next<T: FromStr>(&mut self) -> T {
            let s = self.iter.next().unwrap();
            if let Ok(v) = s.parse::<T>() {
                v
            } else {
                panic!("Parse error")
            }
        }

        pub fn next_vec_len<T: FromStr>(&mut self) -> Vec<T> {
            let n: usize = self.next();
            self.next_vec(n)
        }

        pub fn next_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.next()).collect()
        }
    }

    pub fn read_string() -> String {
        use std::io::Read;

        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }

    pub fn read_line() -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned()
    }
}

pub mod num {
    use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

    pub trait NumOps<Rhs = Self, Output = Self>:
        Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
        + Neg<Output = Output>
    {
}

    impl<T, Rhs, Output> NumOps<Rhs, Output> for T
    where
        T: Add<Rhs, Output = Output>
            + Sub<Rhs, Output = Output>
            + Mul<Rhs, Output = Output>
            + Div<Rhs, Output = Output>
            + Rem<Rhs, Output = Output>
            + Neg<Output = Output>,
    {
    }

    pub trait Num: PartialEq + NumOps + From<i32> {}

    pub trait Float: Num + Copy + PartialOrd + Neg<Output = Self> {}

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

    pub fn gcd<T: Num + Copy>(a: T, b: T) -> T {
        if a % b == 0.into() {
            a
        } else {
            gcd(b, a % b)
        }
    }

    pub fn lcm<T: Num + Copy>(a: T, b: T) -> T {
        let g = gcd(a, b);
        a / g * b
    }
}

pub mod modular {
    const M: i64 = 1000000007;

    #[derive(Debug, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
    pub struct Modular(i64);

    impl ::std::fmt::Display for Modular {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Modular {
        pub fn new(v: i64) -> Modular {
            Modular(v % M)
        }

        pub fn pow(self, mut r: i64) -> Modular {
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
        pub fn recip(self) -> Modular {
            self.pow(M - 2)
        }
    }

    use std::ops::*;

    impl<T: Into<Modular>> Add<T> for Modular {
        type Output = Modular;
        fn add(self, rhs: T) -> Self::Output {
            Modular::new(self.0 + rhs.into().0)
        }
    }
    impl<T: Into<Modular>> AddAssign<T> for Modular {
        fn add_assign(&mut self, rhs: T) {
            *self = *self + rhs;
        }
    }

    impl<T: Into<Modular>> Sub<T> for Modular {
        type Output = Modular;
        fn sub(self, rhs: T) -> Self::Output {
            Modular::new(self.0 - rhs.into().0 + M)
        }
    }
    impl<T: Into<Modular>> SubAssign<T> for Modular {
        fn sub_assign(&mut self, rhs: T) {
            *self = *self - rhs;
        }
    }

    impl<T: Into<Modular>> Mul<T> for Modular {
        type Output = Modular;
        fn mul(self, rhs: T) -> Self::Output {
            Modular::new(self.0 * rhs.into().0)
        }
    }
    impl<T: Into<Modular>> MulAssign<T> for Modular {
        fn mul_assign(&mut self, rhs: T) {
            *self = *self * rhs;
        }
    }

    impl<T: Into<Modular>> Div<T> for Modular {
        type Output = Modular;
        fn div(self, rhs: T) -> Self::Output {
            self * rhs.into().recip()
        }
    }
    impl<T: Into<Modular>> DivAssign<T> for Modular {
        fn div_assign(&mut self, rhs: T) {
            *self = *self / rhs;
        }
    }

    impl Neg for Modular {
        type Output = Modular;
        fn neg(self) -> Self::Output {
            Modular(0) - self
        }
    }

    impl<T: ::std::convert::Into<i64>> ::std::convert::From<T> for Modular {
        fn from(v: T) -> Self {
            Modular::new(v.into())
        }
    }
}

pub mod union_find {
    pub struct UnionFind(Vec<usize>);

    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            UnionFind((0..n).collect())
        }

        pub fn find(&mut self, i: usize) -> usize {
            if self.0[i] == i {
                i
            } else {
                let p = self.0[i];
                self.0[i] = self.find(p);
                self.0[i]
            }
        }

        pub fn union(&mut self, i: usize, j: usize) {
            let ni = self.find(i);
            let nj = self.find(j);
            self.0[ni] = nj;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn scanner_test() {
        use io::Scanner;

        let mut sc = Scanner::new("1 3.14 Hello");

        assert_eq!(sc.next::<i32>(), 1);
        assert_eq!(sc.next::<f64>(), 3.14);
        assert_eq!(sc.next::<String>(), "Hello");

        // let v: Vec<f64> = (0..10).map(|_| sc.next()).collect();

        // let mut sc = Scanner::new("1 2 3 4 5");
        // let v: Vec<i32> = vec![sc.next(); 5];
        // println!("{:?}", v);
    }

    #[test]
    fn test_modular() {
        use modular::*;

        let x: Modular = 12345678.into();
        let y: Modular = 87654321.into();
        assert_eq!(y * x * x.recip(), y);
    }

    #[test]
    fn union_find_test() {
        use union_find::UnionFind;

        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(0, 4);

        assert_eq!(uf.find(0), uf.find(1));
        assert_eq!(uf.find(2), uf.find(3));
        assert_eq!(uf.find(0), uf.find(4));
        assert_eq!(uf.find(1), uf.find(4));
        assert_ne!(uf.find(0), uf.find(2));
        assert_ne!(uf.find(3), uf.find(4));
    }
}
