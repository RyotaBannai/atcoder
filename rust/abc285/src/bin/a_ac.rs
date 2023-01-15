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
 * A - Edge Checker 2
 *
 * https://atcoder.jp/contests/abc285/tasks/abc285_a
 *
 * 番号がちゃんと割り振られている時の親子関係
 */

// #[fastout]
fn main() {
    input! {
        a: usize,
        b: usize
    }
    if b / 2 == a {
        println!("Yes");
    } else {
        println!("No");
    }
}
