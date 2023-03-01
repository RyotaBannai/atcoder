use itertools::Itertools;
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
 * Caracal vs Monster
 *
 * https://atcoder.jp/contests/abc153/tasks/abc153_d
 */
fn rec(x: usize) -> usize {
    if x == 1 {
        return 1;
    }
    1 + 2 * rec(x / 2)
}
// #[fastout]
fn main() {
    input! {
        h: usize,
    }
    println!("{}", rec(h));
}
