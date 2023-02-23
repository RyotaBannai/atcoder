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
use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
/**
 * Trick or Treat
 *
 * https://atcoder.jp/contests/abc166/tasks/abc166_b
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut s = Set::from_iter(1..=n);
    for _ in 0..k {
        input! {
            d: usize,
            a: [usize; d],
        }
        for x in a {
            s.remove(&x);
        }
    }

    println!("{}", s.len());
}
