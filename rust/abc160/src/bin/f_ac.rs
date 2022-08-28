use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
use std::usize::MAX;
// use superslice::Ext;
use ac_library_rs::modint::ModInt1000000007 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Distributing Integers
 *
 * https://atcoder.jp/contests/abc160/tasks/abc160_f
 *
 * tags: #全方位木dp #木dp #tree_dp #rerooting #組み合わせ
 *
 *
 * 参考
 * ・https://betrue12.hateblo.jp/entry/2020/03/29/001927
 * ・https://maspypy.com/atcoder-%e5%8f%82%e5%8a%a0%e6%84%9f%e6%83%b3-2019-03-28abc-160#toc3
 *
 * mod inv のやり方が難しいから、ac library を使うといい
 *
*/

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
    d: Vec<usize>, // 子部分木にある頂点数も持っておきたい
    c: Vec<Mint>,  // 頂点 i とした時の子部分木内の頂点の選び方（組み合わせ数）
    fact: Vec<usize>,
}
impl Rec {
    fn new(g: Vec<Vec<Edge>>) -> Self {
        let mo = 1_000_000_007;
        let n = g.len();
        let mut fact = vec![1; n + 1];
        for i in 1..=n {
            fact[i] *= fact[i - 1] * i;
            fact[i] %= mo;
        }
        Self {
            n,
            g,
            d: vec![0; n],
            c: vec![Mint::new(1); n],
            fact,
        }
    }

    // 問題ごとに異なる
    fn merge(a: usize, b: usize) -> usize {
        a + b
    }

    // v: 注目してる頂点、par: v の親の頂点
    fn dfs1(&mut self, v: usize, par: usize) {
        let deg = self.g[v].len();
        for i in 0..deg {
            let e = self.g[v][i];
            if e.to == par {
                continue;
            }
            self.dfs1(e.to, v);
            // 子部分木の頂点数の総和
            self.d[v] = Self::merge(self.d[v], self.d[e.to] + e.cost); // 頂点 i 自身は含めず、子からカウント
        }

        // 全子部分木の個数(ci~n)とそれぞれの組み合わせ（di~n）がわかったら、頂点 v における組み合わせ数を計算
        let mut comb = Mint::new(self.fact[self.d[v]]);
        for i in 0..deg {
            let e = self.g[v][i];
            if e.to == par {
                continue;
            }
            comb *= self.c[e.to];
            comb /= self.fact[self.d[e.to] + 1]; // (子部分木の頂点数 + 自分)!
        }

        // 親の計算をする時には、子の組み合わせ数は全てわかる
        self.c[v] = comb;
    }

    // 各頂点 v を根とした時の組み合わせ数を計算 v: 注目してる頂点 par: v の親
    fn dfs2(&mut self, v: usize, par: usize) {
        let deg = self.g[v].len();
        let mut comb = Mint::new(self.fact[self.n - 1]); // 初め全体の fact を加えておく

        for i in 0..deg {
            let e = self.g[v][i];
            if e.to == par {
                // 頂点 v の Cpar に対する寄与を取り除いて、親を部分木として数え上げる
                let mut tmp = self.c[par];

                // 新しい全体を加える
                tmp *= self.fact[self.n - 1 - (self.d[v] + 1)];

                // 逆数をかける・わる
                tmp *= self.fact[self.d[v] + 1]; // (部分木の頂点数+自分数)!（重複分）を分母から取り除く
                tmp /= self.fact[self.n - 1]; // 元の全体をから取り除く
                tmp /= self.c[v]; // 自分の組み合わせ数を取り除く

                comb *= tmp; // 親を部分木と見做して、新しい組み合わせ数として加える

                comb /= self.fact[(self.n - 1) - self.d[v]]; // 親を部分木と見做して、その中の頂点を全て同じとして重複分を取り除く
            } else {
                comb *= self.c[e.to];
                comb /= self.fact[self.d[e.to] + 1]; // (子部分木の頂点数+自分)!（重複分）を分母から取り除く
            }
        }

        self.c[v] = comb;

        for i in 0..deg {
            let e = self.g[v][i];
            if e.to == par {
                continue;
            }
            self.dfs2(e.to, v);
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        e: [(usize, usize); n-1]
    }
    let mut g = vec![vec![]; n];
    for (s, t) in e {
        // 無向
        g[s - 1].push(Edge::new(t - 1, 1));
        g[t - 1].push(Edge::new(s - 1, 1));
    }

    let mut rec = Rec::new(g);
    rec.dfs1(0, MAX);
    rec.dfs2(0, MAX);
    for x in rec.c {
        println!("{}", x);
    }
}
