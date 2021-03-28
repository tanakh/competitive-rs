use crate::gf::GF;
use std::{
    any::{Any, TypeId},
    fmt::{self, Display, Formatter},
};

pub struct Vertical<T>(pub Vec<T>);

impl<T> Into<Vertical<T>> for Vec<T> {
    fn into(self) -> Vertical<T> {
        Vertical(self)
    }
}

pub struct Mat<T>(pub Vec<Vec<T>>);

impl<T> Into<Mat<T>> for Vec<Vec<T>> {
    fn into(self) -> Mat<T> {
        Mat(self)
    }
}

pub struct AtCoder<T>(pub T);

macro_rules! impl_atcoder {
    ($t:ty) => {
        impl Display for AtCoder<$t> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

impl_atcoder!(i8);
impl_atcoder!(i16);
impl_atcoder!(i32);
impl_atcoder!(i64);
impl_atcoder!(i128);
impl_atcoder!(isize);
impl_atcoder!(u8);
impl_atcoder!(u16);
impl_atcoder!(u32);
impl_atcoder!(u64);
impl_atcoder!(u128);
impl_atcoder!(usize);

impl Display for AtCoder<f32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.10}", self.0)
    }
}

impl Display for AtCoder<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.20}", self.0)
    }
}

#[test]
fn test_num() {
    assert_eq!(AtCoder(123_u8).to_string(), "123");
    assert_eq!(AtCoder(123_u16).to_string(), "123");
    assert_eq!(AtCoder(123_u32).to_string(), "123");
    assert_eq!(AtCoder(123_u64).to_string(), "123");
    assert_eq!(AtCoder(123_u128).to_string(), "123");
    assert_eq!(AtCoder(123_usize).to_string(), "123");

    assert_eq!(AtCoder(-123_i8).to_string(), "-123");
    assert_eq!(AtCoder(-123_i16).to_string(), "-123");
    assert_eq!(AtCoder(-123_i32).to_string(), "-123");
    assert_eq!(AtCoder(-123_i64).to_string(), "-123");
    assert_eq!(AtCoder(-123_i128).to_string(), "-123");
    assert_eq!(AtCoder(-123_isize).to_string(), "-123");

    assert_eq!(AtCoder(std::f32::consts::PI).to_string(), "3.1415927410");
    assert_eq!(
        AtCoder(std::f64::consts::PI).to_string(),
        "3.14159265358979311600"
    );
}

impl<const P: u64> Display for AtCoder<GF<P>> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl_atcoder!(char);
impl_atcoder!(&str);
impl_atcoder!(String);

#[test]
fn test_str() {
    assert_eq!(AtCoder('A').to_string(), "A");
    assert_eq!(AtCoder("Hello").to_string(), "Hello");
    assert_eq!(AtCoder("World".to_string()).to_string(), "World");
}

impl Display for AtCoder<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.0 { "Yes" } else { "No" })
    }
}

#[test]
fn test_bool() {
    assert_eq!(AtCoder(true).to_string(), "Yes");
    assert_eq!(AtCoder(false).to_string(), "No");
}

impl<T> Display for AtCoder<Vec<T>>
where
    T: Copy,
    AtCoder<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..self.0.len() {
            if i > 0 {
                write!(f, " ")?;
            }
            AtCoder(self.0[i]).fmt(f)?;
        }
        Ok(())
    }
}

#[test]
fn test_vec() {
    assert_eq!(AtCoder(vec![1, 2, 3]).to_string(), "1 2 3");
}

impl<T> Display for AtCoder<Vertical<T>>
where
    T: Copy,
    AtCoder<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..self.0 .0.len() {
            if i > 0 {
                writeln!(f)?;
            }
            AtCoder(self.0 .0[i]).fmt(f)?;
        }
        Ok(())
    }
}

#[test]
fn test_vertical() {
    assert_eq!(AtCoder(Vertical(vec![1, 2, 3])).to_string(), "1\n2\n3");
}

impl<T> Display for AtCoder<Mat<T>>
where
    T: Copy + Any,
    AtCoder<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let r = &self.0 .0;
        for i in 0..r.len() {
            if i > 0 {
                writeln!(f)?;
            }
            for j in 0..r[i].len() {
                if j > 0 && TypeId::of::<char>() != TypeId::of::<T>() {
                    write!(f, " ")?;
                }
                write!(f, "{}", AtCoder(r[i][j]))?;
            }
        }
        Ok(())
    }
}

#[test]
fn test_mat() {
    assert_eq!(
        AtCoder(Mat(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])).to_string(),
        "1 2 3\n4 5 6\n7 8 9"
    );

    assert_eq!(
        AtCoder(Mat(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i']
        ]))
        .to_string(),
        "abc\ndef\nghi"
    );
}

impl<T> Display for AtCoder<Option<T>>
where
    T: Clone,
    AtCoder<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            None => write!(f, "-1"),
            Some(r) => AtCoder(r.clone()).fmt(f),
        }
    }
}

#[test]
fn test_option() {
    assert_eq!(AtCoder(Option::<i32>::None).to_string(), "-1");
    assert_eq!(AtCoder(Some(vec![1, 2, 3])).to_string(), "1 2 3");
}

impl<T, U> Display for AtCoder<Result<T, U>>
where
    T: Clone,
    U: Clone,
    AtCoder<T>: Display,
    AtCoder<U>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Ok(r) => AtCoder(r.clone()).fmt(f),
            Err(r) => AtCoder(r.clone()).fmt(f),
        }
    }
}

#[test]
fn test_result() {
    let t: Result<Vec<i32>, &str> = Ok(vec![1, 2, 3]);
    assert_eq!(AtCoder(t).to_string(), "1 2 3");
    let t: Result<Vec<i32>, &str> = Err("Impossible");
    assert_eq!(AtCoder(t).to_string(), "Impossible");
}
