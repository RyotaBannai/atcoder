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
use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::number::divisor;

/**
 * Disjoint Set of Common Divisors
 *
 * https://atcoder.jp/contests/abc142/tasks/abc142_d
 *
 * tags: #約数 #divisors
 *
 * - a,b の共通の約数から始める.
 * - 約数のset から一つずつ取り出して、そのどの要素でも割り切れない要素を残す、操作をする
 *
 */
// #[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut set = Set::new();
    let x = Set::from_iter(divisor(a).into_iter());
    let y = Set::from_iter(divisor(b).into_iter());
    for i in x {
        if y.contains(&i) {
            set.insert(i);
        }
    }

    let mut ans = set.iter().cloned().collect_vec();
    for p in set.into_iter().rev() {
        if p == 1 {
            continue;
        }
        ans = ans
            .into_iter()
            .filter(|&x| x % p != 0 || x == p)
            .collect_vec();
    }

    println!("{:?}", ans.len());
}
