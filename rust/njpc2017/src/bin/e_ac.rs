use proconio::{fastout, input, marker::Chars};
use std::usize::MAX;
// use std::cmp::{max, min};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 限界集落
 *
 * https://atcoder.jp/contests/njpc2017/tasks/njpc2017_e
 *
 * tags: #全方位木dp #木dp #tree_dp #rerooting #cum_sum #累積和
 *
 * 参考
 * https://ei1333.hateblo.jp/entry/2017/04/10/224413
 *
 * 辺を変える時のカウントを
 * latte += 親までのカウント - 子部分木のカウント - e.dire としても良い　e.dire 1: 辺の向 0: 辺の逆向き（辺が伸びてない）
 * この場合、collection/src/bin/dp/all_directions_tree_dp.rs のように、dp を使わない全方位木dpを使うとよい
 * 親辺の
 * for(edge &e : g[idx]) {
 *  if(e.to == par) { ... }
 *  ...
 * }
 * では、v->par への向きは逆になるのであれば、d.dire == 0 だから余分にカウントすることはない
 *
 * ある頂点 v の全ての部分木のコストが d 以下でないといけないと考える.
 *
*/

#[derive(Clone, Copy, Debug)]
struct Edge {
    to: usize, // この頂点から出て行く頂点番号
    cost: usize,
    dire: isize, // 有効なら 1 反対なら 0
}
impl Edge {
    fn new(to: usize, cost: usize, dire: isize) -> Self {
        Self { to, cost, dire }
    }
}

struct Rec {
    n: usize,
    g: Vec<Vec<Edge>>,
    dp: Vec<Vec<usize>>, // dp[v][i]: v から出る i 番目の有効辺に対する部分木の dp
    weight: Vec<isize>,
    ans: Vec<usize>, // ans[v]: 頂点 v を根とする木の答え
}
impl Rec {
    fn new(g: Vec<Vec<Edge>>) -> Self {
        let n = g.len();
        let mut dp = vec![vec![]; n];
        for i in 0..n {
            dp[i] = vec![0; g[i].len()]; // 必要な分だけ確保
        }
        Self {
            n,
            g,
            dp,
            weight: vec![0; n],
            ans: vec![MAX; n],
        }
    }

    // 問題ごとに異なる // 最大距離ならば、max, 最小なら min などなど
    fn merge(a: usize, b: usize) -> usize {
        a.max(b)
    }
    fn dfs1(&mut self, v: usize, par: usize) -> usize {
        let mut ma = 0;
        let deg = self.g[v].len();
        for i in 0..deg {
            let e = self.g[v][i];
            if e.to == par {
                continue;
            }
            self.dp[v][i] = self.dfs1(e.to, v) + e.cost;
            ma = Self::merge(ma, self.dp[v][i]);
            self.weight[v] += self.weight[e.to] + e.dire;
        }
        ma
    }

    // d_par: distance parent, 注目してる v の親を計算するまでに最大だった距離を再帰的に渡す
    fn dfs2(&mut self, v: usize, d_par: usize, par: usize) {
        let deg = self.g[v].len();
        let mut par_i = 0; // par_i は必ず更新される
        for i in 0..deg {
            let e = self.g[v][i];
            if e.to == par {
                self.dp[v][i] = d_par;
                par_i = i;
            }
        }

        // 根は dfs1 で計算してるから処理しない
        if par != MAX {
            // par->v に向いてる辺なら根に対して逆辺に変えた時に余分にカウントしているから -1
            // v->par に向いてる辺なら根に対して逆辺に変えた時にカウントしてないから +1
            self.weight[v] = self.weight[par] + if self.g[v][par_i].dire == 0 { -1 } else { 1 };
        }

        let (mut dp_l, mut dp_r) = (vec![0; deg + 1], vec![0; deg + 1]);

        for i in 0..deg {
            dp_l[i + 1] = Self::merge(dp_l[i], self.dp[v][i]);
        }
        for i in (0..deg).rev() {
            dp_r[i] = Self::merge(dp_r[i + 1], self.dp[v][i]);
        }

        let mut d_child = self.dp[v].clone();
        d_child.sort_by(|a, b| b.cmp(a));
        d_child.push(0); // 番兵 // 頂点が２つ以上あれば、出次数１は保証される
        self.ans[v] = d_child[0];

        for i in 0..deg {
            let e = self.g[v][i];
            if e.to == par {
                continue;
            }
            self.dfs2(e.to, Self::merge(dp_l[i], dp_r[i + 1]) + e.cost, v);
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        e: [(usize, usize, usize); n-1]
    }
    let mut g = vec![vec![]; n];
    for (a, b, c) in e {
        // 無向
        g[a - 1].push(Edge::new(b - 1, c, 1));
        g[b - 1].push(Edge::new(a - 1, c, 0));
    }

    let mut rec = Rec::new(g);
    rec.dfs1(0, MAX);
    rec.dfs2(0, 0, MAX);

    // println!("{:?}", &rec.ans);
    // println!("{:?}", &rec.weight);
    let mut mi = MAX;
    for (i, &x) in rec.ans.iter().enumerate() {
        if x <= d {
            mi = mi.min(rec.weight[i] as usize);
        }
    }
    if mi == MAX {
        println!("-1");
    } else {
        println!("{}", mi);
    }
}
