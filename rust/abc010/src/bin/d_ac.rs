use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
use std::io;
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
 * D - 浮気予防
 *
 * https://atcoder.jp/contests/abc010/tasks/abc010_4
 *
 * tags: #最小カット
 *
 * https://algo-logic.info/ford-fullkerson/
 * https://algo-logic.info/minimum-cut/
 *
 * 最大流問題を解くと最小カット数がわかる.
 * シンク t に入る量は、その経路の最大流 f に一致するため,
 * この最大流分経路（各経路の流量は 1）をカットすることで、ターゲットの people への経路を全て遮断することができる. またこれが最小カット数となる
 *
 * 例：
 * 最大流 1 で、1 をカットして、2,3 への経路を断つ
 *       - 2  
 * 0 - 1  　　- t
 *       - 3
 *
 *   - 1
 * 0     - t
 *   - 2
 * 最大流 2 で、1,2 をカットして、シンク t への経路を断つ
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
        // println!("{}", v);
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
            // println!("** {}", f);
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
        n: usize, // 高橋くん 0 は n に織り込み済み
        g: usize, // マークしてる数
        e: usize, // sns の関係数
        ppl: [usize; g],
        m: [(usize, usize); e]
    }

    let mut graph = Graph::new(n + 1); // n 人 + マークしてる g 人から入る シンク t 分
    for (a, b) in m {
        graph.add_edge(a, b, 1);
        graph.add_edge(b, a, 1);
    }

    for per in ppl {
        graph.add_edge(per, n, 1);
        graph.add_edge(n, per, 1);
    }

    let mut ford = FordFulkerson::new(graph);
    let ans = ford.max_flow(0, n);

    println!("{}", ans);
}
