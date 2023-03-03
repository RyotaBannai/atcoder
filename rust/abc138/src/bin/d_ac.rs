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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Ki
 *
 * https://atcoder.jp/contests/abc138/tasks/abc138_d
 *
 * tags: #dfs
 *
 * 初めに頂点に追加したいx を追加してから、最後にdfs流す.
 *
 *
 */
struct Rec {
    lis: Vec<Vec<usize>>,
    count: Vec<usize>,
    used: Vec<bool>,
}
impl Rec {
    fn new(lis: Vec<Vec<usize>>) -> Self {
        let n = lis.len();
        Self {
            lis,
            used: vec![false; n],
            count: vec![0; n],
        }
    }
    fn rec(&mut self, u: usize) {
        self.used[u] = true;
        for y in self.lis[u].clone() {
            if !self.used[y] {
                self.count[y] += self.count[u]; // 親からのカウントを流す
                self.rec(y);
            }
        }
    }
}

// #[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n-1],
        qx: [(usize, usize); q],
    }
    let mut lis = vec![vec![]; n + 1];
    for (a, b) in ab {
        lis[a].push(b);
        lis[b].push(a);
    }

    let mut rec = Rec::new(lis);
    for (q, x) in qx {
        rec.count[q] += x;
    }

    rec.rec(1);
    for i in 1..=n {
        print!("{} ", rec.count[i]);
    }
}
