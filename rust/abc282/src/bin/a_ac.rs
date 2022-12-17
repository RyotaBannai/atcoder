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
 *
 * A - Generalized ABC
 *
 * https://atcoder.jp/contests/abc282/tasks/abc282_a
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize
    }
    for i in 0..n {
        print!("{}", (65 + i as u8) as char);
    }
}
