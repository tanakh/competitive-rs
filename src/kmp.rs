pub struct KMP<'a, T> {
    pat: &'a [T],
    fail: Vec<isize>,
}

impl<'a, T: PartialEq> KMP<'a, T> {
    pub fn new(pat: &'a [T]) -> Self {
        let mut fail = vec![-1_isize; pat.len() + 1];
        let mut j = -1_isize;
        for i in 1..=pat.len() {
            while j >= 0 && pat[j as usize] != pat[(i - 1) as usize] {
                j = fail[j as usize];
            }
            j += 1;
            fail[i] = j;
        }
        KMP { pat, fail }
    }

    pub fn find(&self, s: &[T]) -> Option<usize> {
        let m = self.pat.len() as isize;
        let mut k = 0_isize;
        for i in 0..s.len() {
            while k >= 0 && self.pat[k as usize] != s[i] {
                k = self.fail[k as usize];
            }
            k += 1;
            if k >= m {
                return Some(i + 1 - m as usize);
            }
        }
        None
    }

    pub fn find_all(&self, s: &[T]) -> Vec<usize> {
        let m = self.pat.len() as isize;
        let mut ret = vec![];
        let mut k = 0_isize;
        for i in 0..s.len() {
            while k >= 0 && self.pat[k as usize] != s[i] {
                k = self.fail[k as usize];
            }
            k += 1;
            if k >= m {
                ret.push(i + 1 - m as usize);
                k = self.fail[k as usize];
            }
        }
        ret
    }
}

/// Find first needle in haystack
pub fn kmp_find<T: Clone + PartialEq>(haystack: &[T], needle: &[T]) -> Option<usize> {
    KMP::new(needle).find(haystack)
}

/// Find all needle in haystack
pub fn kmp_find_all<T: Clone + PartialEq>(haystack: &[T], needle: &[T]) -> Vec<usize> {
    KMP::new(needle).find_all(haystack)
}

#[test]
fn test_kmp() {
    assert_eq!(kmp_find_all(b"abcdefabcdef", b"abc"), &[0, 6]);
    assert_eq!(kmp_find_all(b"abcdefabcdef", b"xxx"), &[]);
    assert_eq!(kmp_find_all(b"aaaaaa", b"aaa"), &[0, 1, 2, 3]);
}
