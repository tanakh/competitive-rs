use proconio::marker::Usize1;
use proconio::source::{Readable, Source};
use std::io::BufRead;
use std::marker::PhantomData;

/// Read marker for undirected adjacency list graph
///
/// The result type is `Vec<Vec<usize>>`
///
/// It reads input like below:
///
/// ```ignore
/// n:usize m:usize
/// u_1:IndexType v_1:IndexType
/// ...
/// u_m:IndexType v_m:IndexType
/// ```
pub struct ListGraph<IndexType = Usize1>(PhantomData<IndexType>);

impl<IndexType: Readable<Output = usize>> Readable for ListGraph<IndexType> {
    type Output = Vec<Vec<usize>>;

    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output {
        let n = usize::read(source);
        let m = usize::read(source);
        let mut g = vec![vec![]; n];

        for _ in 0..m {
            let u = IndexType::read(source);
            let v = IndexType::read(source);
            g[u].push(v);
            g[v].push(u);
        }

        g
    }
}

/// Read marker for undirected adjacency matrix graph
///
/// The result type is `Vec<Vec<bool>>`
///
/// It reads input like below:
///
/// ```ignore
/// n:usize m:usize
/// u_1:IndexType v_1:IndexType
/// ...
/// u_m:IndexType v_m:IndexType
/// ```
pub struct MatGraph<IndexType = Usize1>(PhantomData<IndexType>);

impl<IndexType: Readable<Output = usize>> Readable for MatGraph<IndexType> {
    type Output = Vec<Vec<bool>>;

    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output {
        let n = usize::read(source);
        let m = usize::read(source);
        let mut g = vec![vec![false; n]; n];

        for _ in 0..m {
            let u = IndexType::read(source);
            let v = IndexType::read(source);
            g[u][v] = true;
            g[v][u] = true;
        }

        g
    }
}

pub struct ListTree<IndexType = Usize1>(PhantomData<IndexType>);

impl<IndexType: Readable<Output = usize>> Readable for ListTree<IndexType> {
    type Output = Vec<Vec<usize>>;

    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output {
        let n = usize::read(source);
        let mut g = vec![vec![]; n];

        for _ in 0..n - 1 {
            let u = IndexType::read(source);
            let v = IndexType::read(source);
            g[u].push(v);
            g[v].push(u);
        }

        g
    }
}

/*
fn visit(
    g: &Vec<Vec<usize>>,
    v: usize,
    scc: &mut Vec<Vec<usize>>,
    s: &mut VecDeque<usize>,
    ins: &mut Vec<bool>,
    low: &mut Vec<usize>,
    num: &mut Vec<usize>,
    time: usize,
) {
    low[v] = time;
    num[v] = time;

    s.push_back(v);
    ins[v] = true;

    for &e in g[v].iter() {
        let w = e;
        if num[w] == 0 {
            visit(g, w, scc, s, ins, low, num, time + 1);
            low[v] = min(low[v], low[w]);
        } else if ins[w] {
            low[v] = min(low[v], num[w]);
        }
    }

    if low[v] == num[v] {
        let mut c = vec![];
        loop {
            let w = s.pop_back().unwrap();
            ins[w] = false;
            c.push(w);
            if v == w {
                break;
            }
        }
        scc.push(c);
    }
}

pub fn strongly_connected_components(g: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = g.len();

    let mut num = vec![0; n];
    let mut low = vec![0; n];
    let mut s = VecDeque::new();
    let mut ins = vec![false; n];
    let mut scc = vec![];

    for u in 0..n {
        if num[u] == 0 {
            visit(g, u, &mut scc, &mut s, &mut ins, &mut low, &mut num, 1);
        }
    }

    scc
}
*/
