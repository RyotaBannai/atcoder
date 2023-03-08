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
 * Go to School
 *
 * https://atcoder.jp/contests/abc142/tasks/abc142_c
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    for (_, x) in a.into_iter().enumerate().map(|(a, b)| (b, a)).sorted() {
        print!("{} ", x + 1);
    }
}
