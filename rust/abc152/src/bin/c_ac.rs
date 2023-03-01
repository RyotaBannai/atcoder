use library::chmin;
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
 *
 * Low Elements
 *
 * https://atcoder.jp/contests/abc152/tasks/abc152_c
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [usize; n]
    }
    let mut count = 0;
    let mut mi = std::usize::MAX;
    for x in xs {
        if x <= mi {
            count += 1;
        }
        chmin!(mi, x);
    }
    println!("{}", count);
}
