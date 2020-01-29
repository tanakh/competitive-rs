use std::ops::{Div, Mul, MulAssign};

/// Calculate factorial
pub fn fact<T: MulAssign<T> + From<i32>>(n: usize) -> T {
    let mut ret = 1.into();
    for i in 2..=n {
        ret *= T::from(i as i32);
    }
    ret
}

/// number of k-combinations
pub fn comb<T: Mul<Output = T> + Div<Output = T> + From<i32>>(n: usize, k: usize) -> T {
    let mut n = n;
    let mut k = k;
    let mut ret = 1.into();
    while k > 0 {
        ret = ret * T::from(n as i32) / T::from(k as i32);
        n -= 1;
        k -= 1;
    }
    ret
}

/// Calculate k-multicombination
pub fn multicomb<T: Mul<Output = T> + Div<Output = T> + From<i32>>(n: usize, k: usize) -> T {
    comb(n + k - 1, k)
}

/// Generate factorial table
pub fn gen_fact_table<T: Mul<Output = T> + From<i32> + Clone>(n: usize) -> Vec<T> {
    let mut ret = vec![T::from(1); n];
    for i in 2..n {
        ret[i] = ret[i - 1].clone() * T::from(i as i32);
    }
    ret
}

/// number of k-combinations using factorial table
pub fn comb_from_table<T: Mul<Output = T> + Div<Output = T> + Clone>(
    n: usize,
    k: usize,
    fact: &[T],
) -> T {
    fact[n].clone() / (fact[k].clone() * fact[n - k].clone())
}

/// Calculate k-multicombination using factorial table
pub fn multicomb_from_table<T: Mul<Output = T> + Div<Output = T> + Clone>(
    n: usize,
    k: usize,
    fact: &[T],
) -> T {
    comb_from_table(n + k - 1, k, fact)
}
