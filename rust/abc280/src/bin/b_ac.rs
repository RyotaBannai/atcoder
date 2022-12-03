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
 * B - Inverse Prefix Sum
 *
 * https://atcoder.jp/contests/abc280/tasks/abc280_b
 */

// #[fastout]
fn main() {
    input! {
     n: usize,
     s:[isize; n]
    }
    let mut prev = s[0];
    print!("{} ", prev);
    for &x in s.iter().skip(1) {
        print!("{} ", x - prev);
        prev = x;
    }
}
