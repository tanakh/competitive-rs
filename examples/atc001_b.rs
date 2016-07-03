extern crate competitive;
use competitive::*;

fn main() {
    let (n, q): (usize, usize) = io::readln();
    let qs: Vec<(i32, usize, usize)> = (0..q).map(|_| io::readln()).collect();

    let mut uf = union_find::UnionFind::new(n);
    for (p, a, b) in qs {
        if p == 0 {
            uf.union(a, b);
        } else {
            let pa = uf.find(a);
            let pb = uf.find(b);
            println!("{}", if pa == pb { "Yes" } else { "No" });
        }
    }
}
