use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Unique Color
 *
 * https://atcoder.jp/contests/abc198/tasks/abc198_e
 *
 * tags: #dfs
 *
 * 頂点に入るごとに頂点で使う色を管理する.
 * 頂点に入る時に、すでに使われている（色の数が1以上の時）は「良い頂点」にはならない.
 *
 */

struct Rec {
    list: Vec<Vec<usize>>,
    used: Vec<bool>,
    ans: Set,
    c: Vec<usize>,
    m: Map,
}
impl Rec {
    fn new(list: Vec<Vec<usize>>, c: Vec<usize>) -> Self {
        let n = list.len();
        Self {
            list,
            used: vec![false; n],
            ans: Set::new(),
            c,
            m: Map::new(),
        }
    }
    fn rec(&mut self, u: usize) {
        *self.m.entry(self.c[u - 1]).or_insert(0) += 1;
        if let Some(&x) = self.m.get(&self.c[u - 1]) {
            if x == 1 {
                self.ans.insert(u);
            }
        }

        self.used[u] = true;

        for y in self.list[u].clone() {
            if !self.used[y] {
                self.rec(y);
                self.used[y] = false;
                *self.m.entry(self.c[y - 1]).or_insert(0) -= 1; // 戻ってきた時に使った色を減らす
            }
        }
    }
}

// #[fastout]
fn main() {
    input! {
        n: usize,
        c: [usize; n],
        ab: [(usize, usize); n-1]
    }
    let mut list = vec![vec![]; n + 1];
    for (a, b) in ab {
        list[a].push(b);
        list[b].push(a);
    }

    let mut rec = Rec::new(list, c);
    rec.rec(1);
    for &x in rec.ans.iter() {
        println!("{}", x);
    }
}
