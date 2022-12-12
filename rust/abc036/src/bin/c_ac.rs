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
use library::structure::compress::*;

/**
 * C - 座圧
 *
 * https://atcoder.jp/contests/abc036/tasks/abc036_c
 *
 * tags: #座標圧縮 #compress
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    let na = compress1(a);
    for x in na {
        println!("{}", x);
    }
}
