/**
 * @cpg_dirspec traffic_tree
 *
 * cpg run -p src/bin/dp/aoj_traffic_tree.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
use std::usize::MAX;
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
 * Traffic Tree
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/1595
 *
 * tags: #全方位木dp #木dp #tree_dp #rerooting #cum_sum #累積和
 *
 * ある頂点を始点として全ての頂点に到達するために必要な最小ステップ数を求めよ
 * ただし、ある頂点から1本の辺をたどって別の頂点に移動することを1ステップとする。
 *
 * 行き止まりの場合は、元の頂点（親）まで戻ってこないといけなく、その場合もどる分のステップ数もカウントする
 *
 *
 *
 * 考察
 * 全ての頂点を通って、元の頂点に戻ってくるまでに必要なステップ数は (n-1)*2 とわかる
 * 最後の頂点に到達した時には戻ってくる必要がないため、その分を引きたい
 * この時、最後に到達する頂点が一番遠い時に、最小のステップ数で全ての頂点に到達できるため、部分木が最大の数を (n-1)*2 から引く（ステップ数が多いほど、その分親まで戻ってこなくてはいけない）
 *
*/
use collection::utils::read::*;

#[derive(Clone, Copy, Debug)]
struct Edge {
    to: usize, // この頂点から出て行く頂点番号
    cost: usize,
}
impl Edge {
    fn new(to: usize, cost: usize) -> Self {
        Self { to, cost }
    }
}

struct Rec {
    n: usize,
    g: Vec<Vec<Edge>>,
    dp: Vec<Vec<usize>>, // dp[v][i]: v から出る i 番目の有効辺に対する部分木の dp
    ans: Vec<usize>,     // ans[v]: 頂点 v を根とする木の答え
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
        }
        ma
    }

    // d_par: distance parent, 注目してる v の親を計算するまでに最大だった距離を再帰的に渡す
    fn dfs2(&mut self, v: usize, d_par: usize, par: usize) {
        let deg = self.g[v].len();
        for i in 0..deg {
            let e = self.g[v][i];
            if e.to == par {
                self.dp[v][i] = d_par;
            }
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

// #[fastout]
fn main() {
    let n = read::<usize>()[0];
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let a = read::<usize>();
        let (s, t, w) = (a[0] - 1, a[1] - 1, 1);
        // 無向
        g[s].push(Edge::new(t, w));
        g[t].push(Edge::new(s, w));
    }
    if n == 1 {
        println!("0");
        return;
    }

    let mut rec = Rec::new(g);
    rec.dfs1(0, MAX);
    rec.dfs2(0, 0, MAX);
    for i in 0..n {
        println!("{}", (n - 1) * 2 - rec.ans[i]);
    }
}
