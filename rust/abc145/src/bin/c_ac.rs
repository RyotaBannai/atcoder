use itertools::{Combinations, Itertools};
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
 * Average Length
 *
 * https://atcoder.jp/contests/abc145/tasks/abc145_c
 *
 * tags: #permtation #順列
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(f64,f64);n]
    }
    let mut d = 0.;
    for comb in (0..n).permutations(n) {
        for i in 0..n - 1 {
            let (x1, y1) = xy[comb[i]];
            let (x2, y2) = xy[comb[i + 1]];
            d += ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt();
        }
    }
    for i in 1..=n {
        d /= i as f64;
    }
    println!("{}", d);
}
