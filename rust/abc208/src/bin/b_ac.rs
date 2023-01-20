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
 * Factorial Yen Coin
 *
 * https://atcoder.jp/contests/abc208/tasks/abc208_b
 *
 * tags: #階乗
 *
 * 大きい方から順に引いていく.
 */

// #[fastout]
fn main() {
    input! {
        mut p: usize
    }
    let mut fact = vec![1; 11];
    for i in 1..=10 {
        fact[i] *= fact[i - 1] * i;
    }
    let mut count = 0;
    while p >= 1 {
        for &sub in fact.iter().rev() {
            if p >= sub {
                p -= sub;
                count += 1;
                break;
            }
        }
    }
    println!("{}", count);
}
