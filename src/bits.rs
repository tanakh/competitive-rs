use std::ops::Index;

pub struct BitSet(u32);

const TRUE: bool = true;
const FALSE: bool = false;

impl Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, index: usize) -> &bool {
        if self.0 & (1 << index) != 0 {
            &TRUE
        } else {
            &FALSE
        }
    }
}

impl BitSet {
    pub fn count(&self) -> usize {
        self.0.count_ones() as usize
    }
}

/// Returns power of bitset (n <= 31)
pub fn power_bitset(n: usize) -> impl Iterator<Item = BitSet> {
    assert!(n <= 31);
    PowerBitSetIter {
        cur: 0,
        n: n as u32,
    }
}

struct PowerBitSetIter {
    cur: u32,
    n: u32,
}

impl Iterator for PowerBitSetIter {
    type Item = BitSet;
    fn next(&mut self) -> Option<BitSet> {
        if self.cur < (1 << self.n) {
            let ret = BitSet(self.cur);
            self.cur += 1;
            Some(ret)
        } else {
            None
        }
    }
}
