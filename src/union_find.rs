pub struct UnionFind(Vec<usize>);

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind((0..n).collect())
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.0[i] == i {
            i
        } else {
            let p = self.0[i];
            self.0[i] = self.find(p);
            self.0[i]
        }
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let ni = self.find(i);
        let nj = self.find(j);
        self.0[ni] = nj;
    }
}

#[test]
fn union_find_test() {
    let mut uf = UnionFind::new(5);
    uf.union(0, 1);
    uf.union(2, 3);
    uf.union(0, 4);

    assert_eq!(uf.find(0), uf.find(1));
    assert_eq!(uf.find(2), uf.find(3));
    assert_eq!(uf.find(0), uf.find(4));
    assert_eq!(uf.find(1), uf.find(4));
    assert_ne!(uf.find(0), uf.find(2));
    assert_ne!(uf.find(3), uf.find(4));
}
