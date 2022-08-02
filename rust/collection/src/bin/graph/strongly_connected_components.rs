/**
 * @cpg_dirspec strongly_connected_components
 *
 * cpg run -p src/bin/graph/strongly_connected_components.rs
 */
use std::io;
use std::usize::MAX;
// use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 強連結成分分解
 *
 * tags: #強連結成分分解 #strongly_connected_components
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/GRL_3_C
 *
 * ・1回目の dfs で帰り順を決める
 * ・有向辺の逆辺を用いて、帰り順が大きい頂点から dfs を行い行けるところまで行けるグループが同じ成分となる
 *
 * 考え方
 * 1. 強連結成分分解は、辺の向きを逆にしたグラフで行き来できる成分かどうかで求めることができる。１つ目の成分がもとまったら、次の帰り順が大きい頂点から始めて同処理を行う
 * 2. 1 で探索する辺を元の先頭から始めたい（初めの帰り順が大き方から）ため、帰り順を先に求める
 *
 * 参考
 * ・https://pione.hatenablog.com/entry/2021/03/11/232159
 * ・https://manabitimes.jp/math/1250
 *
 *
 * SCC
 * ・https://atcoder.jp/contests/practice2/tasks/practice2_g?lang=ja
 * 2-SAT
 * ・https://pione.hatenablog.com/entry/2021/03/12/002204
 * 有向グラフ問題まとめ
 * ・https://blog.hamayanhamayan.com/entry/2017/06/11/124732
 *
 */

struct Rec {
    n: usize,              // グラフ全体の頂点数
    list: Vec<Vec<usize>>, // 木の連接リスト
    rev: Vec<Vec<usize>>,  // 木の逆順連接リスト
    used: Vec<bool>,       // 強連結成分分解の探索において、成分分解された頂点かどうか
    ord: Vec<usize>,       // １回目の dfs での帰りがけの順に頂点を入れる
    component: Vec<usize>,
}
impl Rec {
    fn new(list: Vec<Vec<usize>>) -> Self {
        let n = list.len();
        let mut rev = vec![vec![]; n];
        for (u, xs) in list.iter().enumerate() {
            for &x in xs {
                rev[x].push(u);
            }
        }

        Self {
            n,
            list,
            rev,
            used: vec![false; n],
            ord: vec![],
            component: vec![MAX; n],
        }
    }
    fn make_ord(&mut self) {
        for u in 0..self.n {
            if !self.used[u] {
                self.dfs(u);
            }
        }
    }
    // 帰りがけ順を作る
    fn dfs(&mut self, u: usize) {
        self.used[u] = true;
        for x in self.list[u].clone() {
            if !self.used[x] {
                self.dfs(x);
            }
        }
        self.ord.push(u);
    }

    fn make_comps(&mut self) {
        let mut k = 0;
        // 帰りがけの逆順で探索
        for &x in self.ord.clone().iter().rev() {
            if self.component[x] == MAX {
                self.rev_dfs(x, k);
                k += 1;
            }
        }
    }
    // abc010/src/bin/d_ac.rs
    fn rev_dfs(&mut self, u: usize, k: usize) {
        self.component[u] = k;
        for x in self.rev[u].clone() {
            if self.component[x] == MAX {
                self.rev_dfs(x, k); // 同じグループになるから k をそのまま割り当てる
            }
        }
    }

    fn run(&mut self) {
        self.make_ord();
        self.make_comps();
    }
}

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (v, e) = (a[0], a[1]);
    let mut list = vec![vec![]; v]; // 連接リスト
    for _ in 0..e {
        let b = read::<usize>();
        let (s, t) = (b[0], b[1]);
        list[s].push(t); // 有向
    }

    let mut rec = Rec::new(list);
    rec.run();

    let q = read::<usize>()[0];
    for _ in 0..q {
        let c = read::<usize>();
        let (x, y) = (c[0], c[1]);
        let ans = if rec.component[x] == rec.component[y] {
            1
        } else {
            0
        };
        println!("{}", ans);
    }
}
