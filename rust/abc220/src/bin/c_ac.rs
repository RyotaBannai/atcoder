use proconio::{fastout, input, marker::Chars};
use superslice::Ext;
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
 * C - Long Sequence
 *
 * https://atcoder.jp/contests/abc220/tasks/abc220_c
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize
    }
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }
    let ma = sum[n];
    let p = x / ma;
    let rest = x % ma;
    let pos = sum.upper_bound(&rest); // 累積和のindex-0 を 0 で余分に確保してるからpos をそのまま使う.
    println!("{}", p * n + pos);
}
