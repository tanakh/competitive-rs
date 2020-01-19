use std::cmp::min;
use std::collections::VecDeque;

pub fn visit(
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
