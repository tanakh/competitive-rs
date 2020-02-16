use std::ops::{Add, Div};

/// Generic binary search
///
/// # Arguments
///
/// * `l` - the value assumes to `pred(l) = true`
/// * `r` - the value assumes to `pred(r) = false`
/// * `pred` - predicate for binary search
///
/// # Returns
///
/// * `ret` where `pred(ret) = true` && `pred(ret + delta) = false`
///
/// # Note
///
/// `pred(l)` and `pred(r)` are not called. `pred` is called only values in the range `(l, r)`.
///
pub fn binary_search<T: Add<Output = T> + Div<Output = T> + PartialEq + From<u8> + Copy>(
    l: T,
    r: T,
    pred: impl Fn(T) -> bool,
) -> T {
    let mut l = l;
    let mut r = r;
    let two = T::from(2_u8);
    loop {
        let m = (l + r) / two;
        if l == m || r == m {
            break l;
        }
        if pred(m) {
            l = m;
        } else {
            r = m;
        }
    }
}

#[test]
fn binary_search_test() {
    let v = [1, 2, 3, 4, 5];
    assert_eq!(3, binary_search(v.len() as _, -1, |i| v[i as usize] > 3));
    assert_eq!(5, binary_search(v.len() as _, -1, |i| v[i as usize] > 100));
    assert_eq!(0, binary_search(v.len() as _, -1, |i| v[i as usize] > 0));
}

/// Returns index to the first element in `v` which does not compare less than `val`.
pub fn lower_bound<T: PartialOrd>(v: &[T], val: &T) -> usize {
    (binary_search(-1, v.len() as i64, |i: i64| v[i as usize] < *val) + 1) as usize
}

#[test]
fn lower_bound_test() {
    let v: &[i32] = &[1, 3, 3, 4, 5];
    assert_eq!(lower_bound(v, &0), 0);
    assert_eq!(lower_bound(v, &1), 0);
    assert_eq!(lower_bound(v, &2), 1);
    assert_eq!(lower_bound(v, &3), 1);
    assert_eq!(lower_bound(v, &4), 3);
    assert_eq!(lower_bound(v, &5), 4);
    assert_eq!(lower_bound(v, &999), 5);
}

/// Returns index to the first element in `v` which compares greater than `val`.
pub fn upper_bound<T: PartialOrd>(v: &[T], val: &T) -> usize {
    (binary_search(-1, v.len() as i64, |i: i64| v[i as usize] <= *val) + 1) as usize
}

#[test]
fn upper_bound_test() {
    let v: &[i32] = &[1, 3, 3, 4, 5];
    assert_eq!(upper_bound(v, &0), 0);
    assert_eq!(upper_bound(v, &1), 1);
    assert_eq!(upper_bound(v, &2), 1);
    assert_eq!(upper_bound(v, &3), 3);
    assert_eq!(upper_bound(v, &4), 4);
    assert_eq!(upper_bound(v, &5), 5);
    assert_eq!(upper_bound(v, &999), 5);
}

/// Returns `(lower_bound(v, val), upper_bound(v, val))`
pub fn equal_range<T: PartialOrd>(v: &[T], val: &T) -> (usize, usize) {
    (lower_bound(v, val), upper_bound(v, val))
}
