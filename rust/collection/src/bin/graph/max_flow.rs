/**
 * @cpg_dirspec max_flow
 *
 * cpg run -p src/bin/graph/max_flow.rs
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
    from: usize,
    to: usize,
    rev: usize, // 逆流
    cap: usize, // 容量
}

impl Edge {
    fn new(from: usize, to: usize, rev: usize, cap: usize) -> Self {
        Self { from, to, rev, cap }
    }
}

#[derive(Debug, Clone)]
struct Graph {
    pub edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            edges: vec![vec![]; n],
        }
    }
    fn len(&self) -> usize {
        self.edges.len()
    }
    // 有向辺を加える
    fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
        let t_idx = self.edges[to].len();
        let f_idx = self.edges[from].len();
        // rev の index は有向辺の双方向を管理するために利用.
        // to からの rev は有向辺だから本来は存在しない流れ
        self.edges[from].push(Edge::new(from, to, t_idx, cap));
        self.edges[to].push(Edge::new(to, from, f_idx, 0));
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
        let edges = &mut self.g.edges[v].clone();
        for (i, e) in edges.iter().enumerate() {
            if e.cap > 0 && !self.used[e.to] {
                let d = self.dfs(e.to, t, min(f, e.cap));
                if d > 0 {
                    // cap を減らして(頂点 i の量(d))逆流を増やす
                    self.g.edges[v][i].cap -= d;
                    // 自己ループはないと仮定(あれば、g[e.to][e.rev +1])とする必要がある
                    self.g.edges[e.to][e.rev].cap += d;
                    return d;
                }
            }
        }
        // 進行できるパスがない
        0
    }
    // s-t 間の最大流
    fn max_flow(&mut self, s: usize, t: usize) -> usize {
        let mut flow = 0;
        let l = self.g.len();
        // 進行できるパスが無くなるまで繰り返す
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

// use std::io;

// aoj
// １行読み込んで、空白区切りで vec にして返す
// fn read<T: std::str::FromStr>() -> Vec<T> {
//     let mut buf = String::new();
//     io::stdin().read_line(&mut buf).unwrap();
//     buf.trim().split(' ').flat_map(str::parse).collect()
// }

#[fastout]
fn main() {
    input! {
      n: usize,
      e: usize,
      c: [(usize, usize, usize); e]
    }

    // aoj
    // let inp = read::<usize>();
    // let (n, e) = (inp[0], inp[1]);

    let mut g = Graph::new(n);

    // 頂点ひとつが cap をもつのではなく、u-v 間で cap をもつ.
    // u-w 間は別の cap だから、u は v+w 分流すことができる.
    for (u, v, cap) in c {
        g.add_edge(u, v, cap);
    }

    // aoj
    // for _ in 0..e {
    //     let inp = read::<usize>();
    //     let (u, v, cap) = (inp[0], inp[1], inp[2]);
    //     g.add_edge(u, v, cap);
    // }

    let mut ford = FordFulkerson::new(g);
    let ans = ford.max_flow(0, n - 1);

    println!("{}", ans);
}
