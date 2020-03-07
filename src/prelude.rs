// re-export frequently used std items
pub use std::cmp::{max, min, Reverse};
pub use std::collections::{BTreeMap, BTreeSet, VecDeque};
pub use std::{i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize};

pub use itertools::Itertools;
pub use memoise::memoise;
pub use typenum_promote::promote;

// input! macro
pub use proconio::input;
pub use proconio::marker::{Bytes, Chars, Isize1, Usize1};

// num stuffs
pub use num::complex::Complex;
pub use num::integer::{gcd, lcm};
pub use num::{abs, clamp, BigInt, BigRational, BigUint, Num, Rational};

// comprehension
pub use comprehension::*;

// macros
pub use crate::echo;

// re-exports
pub use crate::binary_search::{binary_search, lower_bound, upper_bound};
pub use crate::bits::{power_bitset, SmallBitSet};
pub use crate::gf::GF;
pub use crate::inf::{MaybeInf, MaybeInf::*};
pub use crate::ix::{Board, Ix2};
pub use crate::util::{tf, yn, Echo, Mat, MatS};
