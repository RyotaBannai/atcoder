/**
 * @cpg_dirspec max_flow
 */
use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 最大流
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/GRL_6_A
 *
 * tags: #max_flow #最大流 #FordFullkerson
 *
 * https://algo-logic.info/ford-fullkerson/
 */

#[derive(Debug, Clone)]
struct Edge {
    rev: usize,
    from: usize,
    to: usize,
    rest: usize, // 残り
    cap: usize,  // 容量
}

impl Edge {
    fn new(rev: usize, from: usize, to: usize, rest: usize, cap: usize) -> Self {
        Self {
            rev,
            from,
            to,
            rest,
            cap,
        }
    }
}

#[derive(Debug, Clone)]
struct Graph {
    pub nodes: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            nodes: vec![vec![]; n],
        }
    }
    fn len(&self) -> usize {
        self.nodes.len()
    }
    // 有向辺を加える ??
    fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
        let tl = self.nodes[to].len();
        let fl = self.nodes[from].len();
        self.nodes[from].push(Edge::new(tl, from, to, cap, cap));
        self.nodes[to].push(Edge::new(fl, to, from, 0, 0));
    }
}

struct FordFulkerson {
    used: Vec<bool>,
    g: Graph,
}

impl FordFulkerson {
    fn new(g: Graph) -> Self {
        Self { used: vec![], g }
    }
    fn dfs(&mut self, v: usize, t: usize, f: usize) -> usize {
        // 最後に到達
        if v == t {
            return f;
        }
        self.used[v] = true;
        let mut nodes_ref = self.g.nodes[v].clone();
        for mut e in nodes_ref.iter_mut() {
            if !self.used[e.to] && e.cap > 0 {
                let d = self.dfs(e.to, t, min(f, e.cap));
                if d > 0 {
                    e.cap -= d;
                    // 逆向きの辺
                    // 自己ループはないと仮定(あれば、g[e.to][e.to +1]) とする必要がある
                    let redge = &mut self.g.nodes[e.to][e.rev];
                    redge.cap += d;
                    self.g.nodes[v] = nodes_ref.clone();
                    return d;
                }
            }
        }
        // 進行できるパスがない
        0
    }

    fn max_flow(&mut self, s: usize, t: usize) -> usize {
        let mut flow = 0;
        let l = self.g.len();
        loop {
            self.used = vec![false; l];
            let f = self.dfs(s, t, std::usize::MAX);
            if f == 0 {
                return flow;
            } else {
                flow += f;
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
      n: usize,
      e: usize,
      c: [(usize, usize, usize); e]
    }

    let mut g = Graph::new(n);
    for (u, v, cap) in c {
        g.add_edge(u, v, cap);
    }

    let mut ford = FordFulkerson::new(g);
    let ans = ford.max_flow(0, 3);

    println!("{}", ans);
}
