use std::io::Read;
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
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    s
}

pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned()
}
