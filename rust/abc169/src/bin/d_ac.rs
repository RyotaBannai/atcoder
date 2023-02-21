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
// type Set = BTreeSet<isize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::number::factorize;

/**
 * Div Game
 *
 * https://atcoder.jp/contests/abc169/tasks/abc169_d
 *
 * tags: #素因数分解
 *
 * 素因数分解は √10^12 = 10^6 で高速で
 * 個数を数える処理も、log2(10^12) <=40 だから十分高速に処理できる
 *
 *
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
    }

    let m = factorize(n);
    let mut ans = 0;
    for (p, mut count) in m.into_iter() {
        for i in 1.. {
            if count < i {
                break;
            }
            count -= i;
            ans += 1;
        }
    }
    println!("{}", ans);
}
