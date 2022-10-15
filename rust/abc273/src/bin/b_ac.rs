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
 * https://atcoder.jp/contests/abc273/tasks/abc273_b
 */

#[fastout]
fn main() {
    input! {
        mut x: f64,
        k: usize
    }

    for _ in 0..k {
        x /= 10.;
        x = x.round();
    }

    for _ in 0..k {
        x *= 10.;
    }
    println!("{}", x);
}
