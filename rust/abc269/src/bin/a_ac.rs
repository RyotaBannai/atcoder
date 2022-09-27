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
 * A - Anyway Takahashi
 *
 * https://atcoder.jp/contests/abc269/tasks/abc269_a
 */

#[fastout]
fn main() {
    input! {
        a:isize,
        b:isize,
        c:isize,
        d:isize,
    }
    println!("{}", (a + b) * (c - d));
    println!("Takahashi");
}
