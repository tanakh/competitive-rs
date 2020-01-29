use num::{pow, One};
use std::ops::{Div, MulAssign, Sub};

/// Make bool table by using Sieve of Eratosthenes
pub fn sieve(n: usize) -> Vec<bool> {
    let mut tbl = vec![true; n];
    tbl[0] = false;
    tbl[1] = false;
    for i in 2..n {
        if tbl[i] {
            let mut j = i + i;
            while j < n {
                tbl[j] = false;
                j += i;
            }
        }
    }
    tbl
}

#[test]
fn sieve_test() {
    let p = sieve(10);
    assert_eq!(
        &[false, false, true, true, false, true, false, true, false, false],
        &p[..]
    );
}

/// Returns all primes less than `n`
pub fn primes(n: usize) -> Vec<usize> {
    let mut tbl = vec![true; n];
    let mut ps = vec![];
    tbl[0] = false;
    tbl[1] = false;
    for i in 2..n {
        if tbl[i] {
            ps.push(i);
            let mut j = i + i;
            while j < n {
                tbl[j] = false;
                j += i;
            }
        }
    }
    ps
}

#[test]
fn primes_test() {
    assert_eq!(primes(20), &[2, 3, 5, 7, 11, 13, 17, 19]);
}

/// Prime factorization
pub fn factor(n: usize) -> Vec<(usize, usize)> {
    let mut n = n;
    let mut ret = vec![];
    let ps = primes((n as f64).sqrt() as usize + 2);
    for p in ps {
        let mut cnt = 0;
        while n % p == 0 {
            n /= p;
            cnt += 1;
        }
        if cnt > 0 {
            ret.push((p, cnt));
        }
    }
    if n > 1 {
        ret.push((n, 1));
    }
    ret
}

#[test]
fn factor_test() {
    assert_eq!(factor(1), &[]);
    assert_eq!(factor(2), &[(2, 1)]);
    assert_eq!(factor(4), &[(2, 2)]);
    assert_eq!(factor(8), &[(2, 3)]);
    assert_eq!(factor(6), &[(2, 1), (3, 1)]);
    assert_eq!(factor(57), &[(3, 1), (19, 1)]);
    assert_eq!(factor(60), &[(2, 2), (3, 1), (5, 1)]);
}

/// Number of divisors
pub fn num_of_divisors(factors: &[(usize, usize)]) -> usize {
    factors.iter().map(|r| r.1 + 1).product()
}

#[test]
fn num_of_divisors_test() {
    assert_eq!(num_of_divisors(&factor(1)), 1);
    assert_eq!(num_of_divisors(&factor(2)), 2);
    assert_eq!(num_of_divisors(&factor(4)), 3);
    assert_eq!(num_of_divisors(&factor(8)), 4);
    assert_eq!(num_of_divisors(&factor(6)), 4);
    assert_eq!(num_of_divisors(&factor(57)), 4);
    assert_eq!(num_of_divisors(&factor(60)), 12);
}

/// Sum of divisors
pub fn sum_of_divisors<
    T: From<usize> + Sub<Output = T> + Div<Output = T> + MulAssign + One<Output = T> + Clone,
>(
    factors: &[(usize, usize)],
) -> T {
    let mut ret = T::from(1_usize);
    for &(p, n) in factors {
        ret *= (pow(T::from(p), n + 1) - T::from(1_usize)) / T::from(p - 1);
    }
    ret
}

#[test]
fn sum_of_divisors_test() {
    assert_eq!(sum_of_divisors::<usize>(&factor(1)), 1);
    assert_eq!(sum_of_divisors::<usize>(&factor(2)), 3);
    assert_eq!(sum_of_divisors::<usize>(&factor(4)), 7);
    assert_eq!(sum_of_divisors::<usize>(&factor(8)), 15);
    assert_eq!(sum_of_divisors::<usize>(&factor(6)), 12);
    assert_eq!(sum_of_divisors::<usize>(&factor(60)), 168);
}

/// Returns all divisors of `n`
pub fn divisors(n: usize) -> Vec<usize> {
    let fs = factor(n);
    let mut ret = vec![];
    gen_divisors(&fs[..], 1, &mut ret);
    ret
}

fn gen_divisors(s: &[(usize, usize)], cur: usize, ret: &mut Vec<usize>) {
    if s.is_empty() {
        ret.push(cur);
        return;
    }

    let mut cur = cur;
    for _ in 0..=s[0].1 {
        gen_divisors(&s[1..], cur, ret);
        cur = cur.wrapping_mul(s[0].0);
    }
}
