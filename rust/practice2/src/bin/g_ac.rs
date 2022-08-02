use proconio::{fastout, input, marker::Chars};
use std::usize::MAX;
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
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * SCC
 *
 * https://atcoder.jp/contests/practice2/tasks/practice2_g?lang=ja
 *
 * tags: #強連結成分分解 #strongly_connected_components
 *
 * 参考
 * https://mmxsrup.hatenablog.com/entry/2016/08/15/204014
 * https://kmyk.github.io/blog/writeups/algo-yukicoder-19/
 * https://note.com/omotiti/n/n6f6c8c85f79a?magazine_key=m4632254cdacf
 *
 */

struct Rec {
    n: usize,                             // グラフ全体の頂点数
    list: Vec<Vec<usize>>,                // 木の連接リスト
    rlist: Vec<Vec<usize>>,               // 木の逆順連接リスト
    used: Vec<bool>,  // 強連結成分分解の探索において、成分分解された頂点かどうか
    ord: Vec<usize>,  // １回目の dfs での帰りがけの順に頂点を入れる
    comp: Vec<usize>, // 成分kに割り当て済かどうか
    components: Vec<Vec<(usize, usize)>>, // 各成分ごとに set を持つ index==k
}
impl Rec {
    fn new(list: Vec<Vec<usize>>) -> Self {
        let n = list.len();
        let mut rlist = vec![vec![]; n];
        for (u, xs) in list.iter().enumerate() {
            for &x in xs {
                rlist[x].push(u);
            }
        }

        Self {
            n,
            list,
            rlist,
            used: vec![false; n],
            ord: vec![],
            comp: vec![MAX; n],
            components: vec![],
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
            if self.comp[x] == MAX {
                self.components.push(vec![]); // 成分kを始めて作る
                self.rdfs(x, k, MAX);
                k += 1;
            }
        }
    }
    fn rdfs(&mut self, u: usize, k: usize, par: usize) {
        self.comp[u] = k; // 使う=DAG
        self.components[k].push((par, u));
        for x in self.rlist[u].clone() {
            if self.comp[x] == MAX {
                self.rdfs(x, k, u); // 同じグループになるから k をそのまま割り当てる
            }
        }
    }

    fn run(&mut self) {
        self.make_ord();
        self.make_comps();
    }
}

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        e: [(usize,usize); m]
    };
    let mut list = vec![vec![]; n]; // 連接リスト
    for (s, t) in e {
        //自己ループは使わない
        if s == t {
            continue;
        }
        list[s].push(t); // 有向
    }

    let mut rec = Rec::new(list);
    rec.run();

    println!("{} ", rec.components.len());
    for xs in rec.components {
        print!("{} ", xs.len());
        for x in xs {
            print!("{} ", x.1);
        }
        println!();
    }
}
