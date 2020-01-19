pub fn binary_search(mut ok: i64, mut ng: i64, pred: impl Fn(i64) -> bool) -> i64 {
    while (ok - ng).abs() > 1 {
        let mi = (ok + ng) / 2;
        if pred(mi) {
            ok = mi;
        } else {
            ng = mi;
        }
    }
    ok
}

#[test]
fn binary_search_test() {
    let v = [1, 2, 3, 4, 5];
    assert_eq!(3, binary_search(v.len() as _, -1, |i| v[i as usize] > 3));
    assert_eq!(5, binary_search(v.len() as _, -1, |i| v[i as usize] > 100));
    assert_eq!(0, binary_search(v.len() as _, -1, |i| v[i as usize] > 0));
}
