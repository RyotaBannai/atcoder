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
use library::utils::read::*;

/**
 * B - Multi Test Cases
 *
 * https://atcoder.jp/contests/abc284/tasks/abc284_b
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
    }
    for _ in 0..n {
        input! {
            t: usize,
            a: [usize; t]
        }
        let mut count = 0;
        for x in a {
            if x % 2 == 1 {
                count += 1;
            }
        }
        println!("{}", count);
    }
}
