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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<(usize, usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * How many?
 *
 * https://atcoder.jp/contests/abc214/tasks/abc214_b
 *
 * s の上限が 100 だから n^3 で良い
 */
// #[fastout]
fn main() {
    input! {
        s: usize,
        t: usize
    }

    let mut set = Set::new();
    for i in 0..=s {
        for j in 0..=s - i {
            for k in 0..=s - j - i {
                if i * j * k <= t {
                    set.insert((i, j, k));
                }
            }
        }
    }
    println!("{}", set.len());
}
