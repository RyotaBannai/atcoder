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
 * C - 1 2 1 3 1 2 1
 *
 * https://atcoder.jp/contests/abc247/tasks/abc247_c
 *
 * tags: #再帰
 *
 */
struct Rec {}
impl Rec {
    fn new() -> Self {
        Self {}
    }
    fn rec(&mut self, n: usize) -> String {
        if n == 1 {
            return n.to_string();
        }
        self.rec(n - 1) + " " + &n.to_string() + " " + &self.rec(n - 1)
    }
}
// #[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut rec = Rec::new();
    println!("{}", rec.rec(n));
}
