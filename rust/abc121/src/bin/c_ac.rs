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
 * C - Energy Drink Collector
 *
 * https://atcoder.jp/contests/abc121/tasks/abc121_c
 *
 * tags: #sort
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        mut m: usize,
        mut ab: [(usize, usize); n]
    }

    ab.sort_unstable();
    let mut sum = 0;
    for (a, b) in ab {
        let mi = m.min(b);
        sum += a * mi;
        m -= mi;
        if m == 0 {
            break;
        }
    }
    println!("{}", sum);
}
