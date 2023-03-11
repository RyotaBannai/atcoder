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
 * Swap Odd and Even
 *
 * https://atcoder.jp/contests/abc293/tasks/abc293_a
 *
 */
// #[fastout]
fn main() {
    input! {
        s: Chars
    }

    for i in 0..s.len() / 2 {
        print!("{}{}", s[2 * i + 1], s[2 * i]);
    }
}
