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
 * A - Rightmost
 *
 * https://atcoder.jp/contests/abc276/tasks/abc276_a
 *
 */

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    for (i, c) in s.iter().rev().enumerate() {
        if c == &'a' {
            println!("{}", s.len() - i);
            return;
        }
    }

    println!("-1");
}
