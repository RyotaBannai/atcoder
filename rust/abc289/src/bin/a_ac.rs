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
 * flip
 *
 * https://atcoder.jp/contests/abc289/tasks/abc289_a
 *
 */

// #[fastout]
fn main() {
    input! {
        s: Chars
    }
    for c in s {
        if c == '0' {
            print!("1");
        } else {
            print!("0");
        }
    }
}
