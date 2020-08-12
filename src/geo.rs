use num::Complex;
use proconio::source::{Readable, Source};
use std::f64;
use std::io::BufRead;

/// Point type
pub type Pt = Complex<f64>;

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

pub fn dot(a: &Pt, b: &Pt) -> f64 {
    (a.conj() * b).re
}

/// Signed area of triangle
pub fn triangle_signed_area(a: &Pt, b: &Pt, c: &Pt) -> f64 {
    cross(&(b - a), &(c - a)) / 2.0
}

/// Area of triangle
pub fn triangle_area(a: &Pt, b: &Pt, c: &Pt) -> f64 {
    triangle_signed_area(a, b, c).abs()
}

/// Cross point of 2 circles
pub fn circle_cross(a: Pt, ar: f64, b: Pt, br: f64) -> Option<(Pt, Pt)> {
    let c = (a - b).norm();

    if c >= ar + br || c + br <= ar || c + ar <= br {
        return None;
    }

    let x = (ar.powi(2) - br.powi(2) + c.powi(2)) / (2.0 * c);
    let h = (ar.powi(2) - x.powi(2)).sqrt();

    let v = (b - a) / c;
    let w = v * Pt::from_polar(1.0, f64::consts::PI / 2.0);
    let c1 = a + x * v + h * w;
    let c2 = a + x * v - h * w;

    Some((c1, c2))
}
