/**
 * @cpg_dirspec bridge
 *
 * cpg run -p src/bin/graph/bridge.rs
 */
// use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
use std::io;
use std::usize::MAX;
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 橋
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/GRL_3_B
 *
 * tags: #橋
 *
 * 参考
 * https://o-treetree.hatenablog.com/entry/2020/06/08/231258#%E9%96%A2%E7%AF%80%E7%82%B9%E3%81%A8%E6%A9%8B
 *
 */

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

struct Rec {
    timer: usize,
    parent: Vec<usize>,
    ord: Vec<usize>,
    low: Vec<usize>,
    list: Vec<Vec<usize>>, // グラフの連接リスト
}
impl Rec {
    fn new(list: Vec<Vec<usize>>) -> Self {
        let n = list.len();
        Self {
            timer: 1,
            parent: vec![MAX; n],
            ord: vec![MAX; n],
            low: vec![MAX; n],
            list,
        }
    }

    // u: 探索中の頂点 pre: 探索前の一つ前の頂点（=親）
    fn dfs(&mut self, u: usize, pre: usize) {
        self.ord[u] = self.timer;
        self.low[u] = self.timer;
        self.timer += 1;

        for &v in &self.list[u].clone() {
            if self.ord[v] == MAX {
                // 未訪問
                self.dfs(v, u);

                self.parent[v] = u;
                self.low[u] = min(self.low[u], self.low[v]);
            } else if self.ord[v] != MAX && v != pre {
                // すでに通った
                self.low[u] = min(self.low[u], self.ord[v]);
            }
        }
    }
}

// #[fastout]
fn main() {
    // aoj inp
    let a = read::<usize>();
    let (n, e) = (a[0], a[1]);
    let mut m = vec![vec![]; n];
    for _ in 0..e {
        let b = read::<usize>();
        let (s, t) = (b[0], b[1]);
        m[s].push(t);
        m[t].push(s);
    }

    let mut rec = Rec::new(m);
    rec.dfs(0, MAX);

    let mut s = Set::new();
    // 関節点のチェックは、各頂点を条件を満たすかどうか見るだけ
    for u in 1..n {
        let p = rec.parent[u];
        if rec.ord[p] < rec.low[u] {
            s.insert(if p > u { (u, p) } else { (p, u) }); // set の時点でソート済にしておく.
        }
    }

    for (a, b) in s {
        println!("{} {}", a, b);
    }
}
