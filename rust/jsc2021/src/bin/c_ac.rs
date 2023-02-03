// use itertools::Itertools;
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
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Max GCD 2
 *
 * https://atcoder.jp/contests/jsc2021/tasks/jsc2021_c
 *
 * tags: #約数
 *
 * a<=x<y<=B の全ての整数の約数を求めた時の数が 2 以上（ある数どうしの共通の約数）となる最大の約数
 *
 * 約数を求める処理がO(√N) だから O(N√N)
 *
 */
use library::{chmax, number::divisor};
// #[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut m = Map::new();
    for x in a..=b {
        let divisors = divisor(x);
        for y in divisors {
            *m.entry(y).or_insert(0) += 1;
        }
    }

    let mut ans = 1;
    for (k, v) in m.into_iter() {
        if v >= 2 {
            chmax!(ans, k);
        }
    }

    println!("{}", ans);
}
