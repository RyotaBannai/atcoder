use library::chmax;
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
 * Bowls and Dishes
 *
 * https://atcoder.jp/contests/abc190/tasks/abc190_c
 *
 * tags: #dfs
 *
 * K=16 だから 2^16 を全探索する
 *
 */
struct Rec {
    ab: Vec<(usize, usize)>,
    cd: Vec<(usize, usize)>,
    used: Vec<usize>,
    ans: usize,
    k: usize,
}
impl Rec {
    fn new(ab: Vec<(usize, usize)>, cd: Vec<(usize, usize)>, k: usize, n: usize) -> Self {
        Self {
            ab,
            cd,
            used: vec![0; n + 1],
            ans: 0,
            k,
        }
    }
    fn rec(&mut self, i: usize) {
        if i == self.k {
            let mut count = 0;
            for &(a, b) in self.ab.iter() {
                if self.used[a] > 0 && self.used[b] > 0 {
                    count += 1;
                }
            }
            chmax!(self.ans, count);
            return;
        }
        self.used[self.cd[i].0] += 1;
        self.rec(i + 1);
        self.used[self.cd[i].0] -= 1;

        self.used[self.cd[i].1] += 1;
        self.rec(i + 1);
        self.used[self.cd[i].1] -= 1;
    }
}

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        k: usize,
        cd: [(usize, usize); k],
    }
    let mut rec = Rec::new(ab, cd, k, n);
    rec.rec(0);
    println!("{}", rec.ans);
}
