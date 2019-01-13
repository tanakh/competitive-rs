/// Input parser.
///
/// # Example
///
/// ```ignore
/// n
/// v_0 v_1 ... v_n-1
/// ```
///
/// This data is parsed by following parser:
///
///
/// ```
/// # #[macro_use] extern crate competitive;
/// input!{
/// # source = "0",
///     n: usize,
///     v: [i64; n],
/// }
/// ```
///
/// # Supported Types
///
/// * FromStr types
/// * `usize1`: 1-origin usize (automatically converted to 0-origin)
/// * (t1, t2, ..., tn): Tuple
/// * `[type; len]`: Array of `type` with `len` (has type Vec<type>)
/// * `chars`: Array of char (converted fron `String`, has type Vec<char>)
/// * `bytes`: Array of byte (converted fron `String`, has type Vec<u8>)
///
#[macro_export]
// #[snippet(name = "input")]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

pub mod io {
    use std;
    use std::str::FromStr;

    /// Input scanner
    ///
    /// # Examples
    ///
    /// ```
    /// # use competitive::io::*;
    /// let mut sc = Scanner::new("1 3.14 Hello");
    /// assert_eq!(sc.next::<i32>(), 1);
    /// assert_eq!(sc.next::<f64>(), 3.14);
    /// assert_eq!(sc.next::<String>(), "Hello");
    /// ```
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
}

pub mod modular {
    const M: i64 = 1000000007;

    #[derive(Debug, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
    pub struct Mod(i64);

    impl ::std::fmt::Display for Mod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Mod {
        pub fn new(v: i64) -> Mod {
            Mod(v % M)
        }

        pub fn pow(self, mut r: i64) -> Mod {
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
        pub fn recip(self) -> Mod {
            self.pow(M - 2)
        }
    }

    use std::ops::*;

    impl<T: Into<Mod>> Add<T> for Mod {
        type Output = Mod;
        fn add(self, rhs: T) -> Self::Output {
            Mod::new(self.0 + rhs.into().0)
        }
    }
    impl<T: Into<Mod>> AddAssign<T> for Mod {
        fn add_assign(&mut self, rhs: T) {
            *self = *self + rhs;
        }
    }

    impl<T: Into<Mod>> Sub<T> for Mod {
        type Output = Mod;
        fn sub(self, rhs: T) -> Self::Output {
            Mod::new(self.0 - rhs.into().0 + M)
        }
    }
    impl<T: Into<Mod>> SubAssign<T> for Mod {
        fn sub_assign(&mut self, rhs: T) {
            *self = *self - rhs;
        }
    }

    impl<T: Into<Mod>> Mul<T> for Mod {
        type Output = Mod;
        fn mul(self, rhs: T) -> Self::Output {
            Mod::new(self.0 * rhs.into().0)
        }
    }
    impl<T: Into<Mod>> MulAssign<T> for Mod {
        fn mul_assign(&mut self, rhs: T) {
            *self = *self * rhs;
        }
    }

    impl<T: Into<Mod>> Div<T> for Mod {
        type Output = Mod;
        fn div(self, rhs: T) -> Self::Output {
            self * rhs.into().recip()
        }
    }
    impl<T: Into<Mod>> DivAssign<T> for Mod {
        fn div_assign(&mut self, rhs: T) {
            *self = *self / rhs;
        }
    }

    impl Neg for Mod {
        type Output = Mod;
        fn neg(self) -> Self::Output {
            Mod(0) - self
        }
    }

    impl<T: ::std::convert::Into<i64>> ::std::convert::From<T> for Mod {
        fn from(v: T) -> Self {
            Mod::new(v.into())
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

pub mod graph {
    use std::cmp::min;
    use std::collections::VecDeque;

    pub fn visit(
        g: &Vec<Vec<usize>>,
        v: usize,
        scc: &mut Vec<Vec<usize>>,
        s: &mut VecDeque<usize>,
        ins: &mut Vec<bool>,
        low: &mut Vec<usize>,
        num: &mut Vec<usize>,
        time: usize,
    ) {
        low[v] = time;
        num[v] = time;

        s.push_back(v);
        ins[v] = true;

        for &e in g[v].iter() {
            let w = e;
            if num[w] == 0 {
                visit(g, w, scc, s, ins, low, num, time + 1);
                low[v] = min(low[v], low[w]);
            } else if ins[w] {
                low[v] = min(low[v], num[w]);
            }
        }

        if low[v] == num[v] {
            let mut c = vec![];
            loop {
                let w = s.pop_back().unwrap();
                ins[w] = false;
                c.push(w);
                if v == w {
                    break;
                }
            }
            scc.push(c);
        }
    }

    pub fn strongly_connected_components(g: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let n = g.len();

        let mut num = vec![0; n];
        let mut low = vec![0; n];
        let mut s = VecDeque::new();
        let mut ins = vec![false; n];
        let mut scc = vec![];

        for u in 0..n {
            if num[u] == 0 {
                visit(g, u, &mut scc, &mut s, &mut ins, &mut low, &mut num, 1);
            }
        }

        scc
    }
}

pub fn binary_search(mut ok: i64, mut ng: i64, pred: impl Fn(i64) -> bool) -> i64 {
    while (ok - ng).abs() > 1 {
        let mi = (ok + ng) / 2;
        if pred(mi) {
            ok = mi;
        } else {
            ng = mi;
        }
    }
    ok
}

#[test]
fn binary_search_test() {
    let v = [1, 2, 3, 4, 5];
    assert_eq!(3, binary_search(v.len() as _, -1, |i| v[i as usize] > 3));
    assert_eq!(5, binary_search(v.len() as _, -1, |i| v[i as usize] > 100));
    assert_eq!(0, binary_search(v.len() as _, -1, |i| v[i as usize] > 0));
}

mod inf {
    use std::cmp::min;
    use std::ops::Add;

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub enum Inf<T> {
        NotInf(T),
        Inf,
    }

    use self::Inf::*;

    impl<T> From<T> for Inf<T> {
        fn from(v: T) -> Self {
            Inf::NotInf(v)
        }
    }

    impl<T: Add<Output = T>> Add for Inf<T> {
        type Output = Inf<T>;
        fn add(self, rhs: Self) -> Self {
            match (self, rhs) {
                (NotInf(a), NotInf(b)) => NotInf(a + b),
                _ => Inf,
            }
        }
    }

    impl<T: Add<Output = T>> Add<T> for Inf<T> {
        type Output = Inf<T>;
        fn add(self, rhs: T) -> Self {
            match self {
                NotInf(a) => NotInf(a + rhs),
                _ => Inf,
            }
        }
    }

    impl<T: Clone + Ord> Inf<T> {
        pub fn min_assign(&mut self, rhs: Self) {
            *self = min(self.clone(), rhs);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn input_macro_simple() {
        input! {
            source = "1 3.14 Hello",
            n: usize,
            f: f64,
            s: String,
        }
        assert_eq!(n, 1);
        assert_eq!(f, 3.14);
        assert_eq!(s, "Hello");
    }

    #[test]
    fn input_macro_vec() {
        input! {
            source = "5\n1 2 3 4 5",
            n: usize,
            v: [usize; n],
        }
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn input_macro_matrix() {
        input! {
            source = "3 4\n1 2 3 4\n5 6 7 8\n9 10 11 12",
            h: usize,
            w: usize,
            v: [[usize; w]; h],
        }
        assert_eq!(v, [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
    }

    #[test]
    fn input_macro_chars() {
        input! {
            source = "string",
            s: chars,
        }
        assert_eq!(s, ['s', 't', 'r', 'i', 'n', 'g']);
    }

    #[test]
    fn input_macro_bytes() {
        input! {
            source = "string",
            s: bytes,
        }
        assert_eq!(s, b"string");
    }

    #[test]
    fn input_macro_char_matrix() {
        input! {
            source = "3 4\n#.#.\n.#.#\n#.#.",
            h: usize,
            _w: usize,
            bd: [chars; h],
        }
        assert_eq!(
            bd,
            [
                ['#', '.', '#', '.'],
                ['.', '#', '.', '#'],
                ['#', '.', '#', '.']
            ]
        );
    }

    #[test]
    fn input_macro_missing_comma() {
        input! {
            source = "1 3.14 Hello",
            n: usize,
            f: f64,
            s: String // allow missing last comma
        }
        assert_eq!(n, 1);
        assert_eq!(f, 3.14);
        assert_eq!(s, "Hello");
    }

    #[test]
    fn test_modular() {
        use modular::*;

        let x: Mod = 12345678.into();
        let y: Mod = 87654321.into();
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
