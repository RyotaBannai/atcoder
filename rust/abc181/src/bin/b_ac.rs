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
 * Trapezoid Sum
 *
 * https://atcoder.jp/contests/abc181/tasks/abc181_b
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }
    let mut sum = 0;
    for (a, b) in ab {
        sum += (b - a + 1) * (a + b) / 2;
    }
    println!("{}", sum);
}
