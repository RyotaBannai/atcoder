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
 * A - Power
 *
 * https://atcoder.jp/contests/abc283/tasks/abc283_a
 *
 */
// #[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut ans = 1;
    for _ in 0..b {
        ans *= a;
    }
    println!("{}", ans);
}
