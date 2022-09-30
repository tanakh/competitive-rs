use std::ops::{Div, Mul, MulAssign, Rem};

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

/// Calculate factorial (memoised)
#[memoise::memoise(n)]
pub fn fact_memo<T>(n: usize) -> T
where
    T: Mul<Output = T> + From<i32> + Clone,
{
    if n == 0 {
        T::from(1 as i32)
    } else {
        fact_memo::<T>(n - 1) * T::from(n as i32)
    }
}

/// number of k-combinations using factorial table
pub fn comb_memo<T: Mul<Output = T> + Div<Output = T> + From<i32> + Clone>(
    n: usize,
    k: usize,
) -> T {
    if n < k {
        0.into()
    } else {
        fact_memo::<T>(n) / (fact_memo::<T>(k) * fact_memo::<T>(n - k))
    }
}

/// Calculate k-multicombination using factorial table
pub fn multicomb_memo<T: Mul<Output = T> + Div<Output = T> + From<i32> + Clone>(
    n: usize,
    k: usize,
) -> T {
    comb_memo::<T>(n + k - 1, k)
}

/// O(log m).
/// Calculate `a.pow(b) % m`
pub fn pow_mod<T>(a: T, b: T, m: T) -> T
where
    T: Eq + Clone + From<u8> + Mul<Output = T> + Div<Output = T> + Rem<Output = T>,
{
    if b == 0.into() {
        1.into()
    } else {
        let t = pow_mod(a.clone(), b.clone() / 2.into(), m.clone());
        let t = t.clone() * t % m;
        if b % 2.into() == 0.into() {
            t
        } else {
            t * a
        }
    }
}
