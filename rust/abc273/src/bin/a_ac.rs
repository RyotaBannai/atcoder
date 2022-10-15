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
 * https://atcoder.jp/contests/abc273/tasks/abc273_a
 */

fn rec(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * rec(n - 1)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize
    }

    println!("{}", rec(n));
}
