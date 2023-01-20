use itertools::Itertools;
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
 * Fair Candy Distribution
 *
 * https://atcoder.jp/contests/abc208/tasks/abc208_c
 *
 * i 番目の数値がソートした時に何番目になるかをあらかじめ計算しておいて、
 * 全ての人数分配布して、全員分なくなった時にK' ソート順で以内になるかどうかを判定する.
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let mut m = vec![0; n];
    for (j, (_, i)) in a
        .clone()
        .iter()
        .enumerate()
        .map(|(i, &a)| (a, i))
        .sorted()
        .enumerate()
    {
        m[i] = j;
    }

    let common = k / n;
    let rest = k % n;
    for x in m {
        let bonus = if x + 1 <= rest { 1 } else { 0 };
        println!("{}", common + bonus);
    }
}
