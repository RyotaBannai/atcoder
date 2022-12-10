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
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::*;
/**
 * D - Max Multiple
 *
 * https://atcoder.jp/contests/abc281/tasks/abc281_d
 *
 * k 回まで流す...
 * すでにk 使っていた時の最大を引き継ぐように for l in 0..=k のようにk まで流す.
 * この回は k+1 にできないから　前回分の最大値を引き継ぐだけにする.
 *
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n]
    }

    // [[商; 余り]; n 回目使った]
    let mut dp = vec![vec![vec![None; d + 1]; k + 1]; n + 1]; // あまり、尚
    dp[0][0][0] = Some(0);

    for i in 0..n {
        for l in 0..=k {
            for j in 0..d {
                if let Some(z) = dp[i][l][j] {
                    let x = a[i]; // i 番目の要素を使う.

                    // x を使う場合
                    if l != k {
                        chmax!(dp[i + 1][l + 1][(z + x) % d], Some(z + x));
                    }

                    // x を使わない
                    chmax!(dp[i + 1][l][j], Some(z));
                }
            }
        }
    }

    // println!("{:?}", &dp[n][k]);
    // for xs in &dp {
    // println!("{:?}", &xs);
    // }

    if let Some(a) = dp[n][k][0] {
        println!("{}", a);
    } else {
        println!("-1");
    }
}
