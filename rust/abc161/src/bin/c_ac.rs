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

use library::min;

/**
 * Replacing Integer
 *
 * https://atcoder.jp/contests/abc161/tasks/abc161_c
 */

// #[fastout]
fn main() {
    input! {
        n: isize,
        k: isize
    }
    let a = n % k;
    println!("{}", min!(a, (a - k).abs()));
}
