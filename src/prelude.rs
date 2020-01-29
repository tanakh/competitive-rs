// re-export frequently used std items
pub use std::cmp::{max, min, Reverse};
pub use std::collections::{BTreeMap, BTreeSet, VecDeque};
pub use std::{i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize};

pub use itertools::Itertools;

// input! macro
pub use proconio::input;
pub use proconio::marker::{Bytes, Chars, Isize1, Usize1};

// num stuffs
pub use num::complex::Complex;
pub use num::integer::{gcd, lcm};
pub use num::{abs, Num};

// macros
pub use crate::def_gf;
pub use crate::echo;
