use ndarray::Array2;
use num::Num;

pub struct PartialSum<T> {
    sum: Vec<T>,
}

impl<T: Num + Copy> PartialSum<T> {
    pub fn new(v: &[T]) -> Self {
        let mut sum = vec![T::zero(); v.len() + 1];
        for i in 0..v.len() {
            sum[i + 1] = sum[i] + v[i];
        }
        Self { sum }
    }

    /// Sum of v[i..j]
    pub fn sum(&self, i: usize, j: usize) -> T {
        self.sum[j] - self.sum[i]
    }
}

#[test]
fn test_partial_sum() {
    let n = 10;
    let v: Vec<usize> = (0..n).collect();
    let psum = PartialSum::new(&v);

    for i in 0..=n {
        for j in i..=n {
            assert_eq!(v[i..j].iter().sum::<usize>(), psum.sum(i, j));
        }
    }
}

pub struct PartialSum2<T> {
    sum: Array2<T>,
}

impl<T: Num + Copy> PartialSum2<T> {
    pub fn new(v: &Array2<T>) -> Self {
        let sh = v.shape();
        let mut sum = Array2::zeros((sh[0] + 1, sh[1] + 1));
        for i in 0..sh[0] {
            for j in 0..sh[1] {
                sum[(i + 1, j + 1)] = sum[(i, j + 1)] + sum[(i + 1, j)] - sum[(i, j)] + v[(i, j)];
            }
        }
        Self { sum }
    }

    /// Sum of v[(i1..i2, j1..j2)]
    pub fn sum(&self, i1: usize, j1: usize, i2: usize, j2: usize) -> T {
        self.sum[(i2, j2)] + self.sum[(i1, j1)] - self.sum[(i1, j2)] - self.sum[(i2, j1)]
    }
}

#[test]
fn test_partial_sum2() {
    let n = 10_usize;

    let mut v = Array2::zeros((n, n));
    for i in 0..n {
        for j in 0..n {
            v[(i, j)] = i * n + j;
        }
    }

    let psum = PartialSum2::new(&v);

    for i1 in 0..=n {
        for j1 in 0..=n {
            for i2 in i1..=n {
                for j2 in j1..=n {
                    let mut sum = 0;
                    for i in i1..i2 {
                        for j in j1..j2 {
                            sum += v[(i, j)];
                        }
                    }
                    assert_eq!(sum, psum.sum(i1, j1, i2, j2));
                }
            }
        }
    }
}
