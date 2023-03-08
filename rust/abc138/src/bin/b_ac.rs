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
 * Resistors in Parallel
 *
 * https://atcoder.jp/contests/abc138/tasks/abc138_b
 *
 * 通分してから求める.
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [f64;n]
    }
    let mut sum = 0.;
    let mul = 1.;
    for &x in &a {
        sum *= x;
    }
    for &x in &a {
        sum += mul / x;
    }

    println!("{}", mul / sum);
}
