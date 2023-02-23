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
// #[fastout]

/**
 * Chat in a Circle
 *
 * https://atcoder.jp/contests/abc173/tasks/abc173_d
 *
 * tags: #二分木
 *
 */

fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    }

    let mut k = n - 1;
    let mut ans = 0;
    for (i, x) in a.into_iter().sorted().rev().enumerate() {
        let mut lim = 2;
        if i == 0 {
            lim -= 1;
        }
        for _ in 0..lim {
            if k > 0 {
                k -= 1;
                ans += x;
            }
        }
    }

    println!("{}", ans);
}
