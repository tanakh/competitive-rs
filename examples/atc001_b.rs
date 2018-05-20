extern crate competitive;
use competitive::*;

fn main() {
    let ss = io::read_string();
    let mut sc = io::Scanner::new(&ss);
    let n: usize = sc.next();
    let q: usize = sc.next();

    let qs: Vec<(i32, usize, usize)> = (0..q).map(|_| (sc.next(), sc.next(), sc.next())).collect();

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
