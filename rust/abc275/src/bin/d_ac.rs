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
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Yet Another Recursive Function
 *
 * https://atcoder.jp/contests/abc275/tasks/abc275_d
 *
 * tags: #recursion #再帰
 */

// #[derive(new)]
struct Rec {
    map: Map,
}
impl Rec {
    fn new() -> Self {
        let mut map = Map::new();
        map.insert(0, 1);
        Self { map }
    }
    fn dfs(&mut self, k: usize) -> usize {
        if let Some(x) = self.map.get(&k) {
            return *x;
        }

        let mut ret = 0;
        ret += self.dfs(k / 2);
        ret += self.dfs(k / 3);
        self.map.insert(k, ret);
        ret
    }
}

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut rec = Rec::new();
    println!("{}", rec.dfs(n));
}
