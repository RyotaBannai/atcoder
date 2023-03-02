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
 * Bounding
 *
 * https://atcoder.jp/contests/abc130/tasks/abc130_b
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        d: [usize; n]
    }
    let mut sum = 0;
    let mut count = 1;
    for di in d {
        sum += di;
        if sum <= x {
            count += 1;
        }
    }
    println!("{}", count);
}
