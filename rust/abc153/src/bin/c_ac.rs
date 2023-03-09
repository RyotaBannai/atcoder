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
 * Fennec vs Monster
 *
 * https://atcoder.jp/contests/abc153/tasks/abc153_c
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        h: [usize; n]
    }

    let mut ans = 0;
    for x in h.into_iter().sorted().rev().skip(k) {
        ans += x;
    }
    println!("{}", ans);
}