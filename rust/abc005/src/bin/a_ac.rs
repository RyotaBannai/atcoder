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
 * A - おいしいたこ焼きの作り方
 *
 * https://atcoder.jp/contests/abc005/tasks/abc005_1
 *
 */

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize
    }
    println!("{}", y / x);
}
