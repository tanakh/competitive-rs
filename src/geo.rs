use num::Complex;
use proconio::source::{Readable, Source};
use std::io::BufRead;

type Pt = Complex<f64>;

/// input marker for Pt
pub struct PtM {}

impl Readable for PtM {
    type Output = Pt;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pt {
        let re = f64::read(source);
        let im = f64::read(source);
        Complex::new(re, im)
    }
}

pub fn cross(a: &Pt, b: &Pt) -> f64 {
    (a.conj() * b).im
}

/// Signed area of triangle
pub fn signed_area(a: &Pt, b: &Pt, c: &Pt) -> f64 {
    cross(&(b - a), &(c - a)) / 2.0
}

/// Area of triangle
pub fn area(a: &Pt, b: &Pt, c: &Pt) -> f64 {
    signed_area(a, b, c).abs()
}
