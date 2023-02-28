use itertools::Itertools;
use library::chmax;
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
 * Dice in Line
 *
 * https://atcoder.jp/contests/abc154/tasks/abc154_d
 *
 * tags: #期待値 #尺取り法
 *
 * 「連続するサイコロ」をk 個取り出す
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n]
    }
    let ma = 1000;
    let mut exp = vec![0.; ma + 1];
    for i in 0..ma {
        exp[i + 1] += exp[i] + i as f64 + 1.;
    }
    for i in 1..=ma {
        exp[i] /= i as f64;
    }

    let mut sum = 0.;
    for i in 0..k {
        sum += exp[p[i]];
    }
    let mut ma = sum;
    for i in k..n {
        let mut s = sum;
        s += exp[p[i]];
        s -= exp[p[i - k]];
        chmax!(ma, s);
        sum = s;
    }
    println!("{}", ma);
}
