use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };

use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Dice Sum
 *
 * https://atcoder.jp/contests/abc248/tasks/abc248_c
 *
 * tags: #DP #動的計画法
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize, // n 個の整数の組み合わせ
        m: usize, // m 以下の非負整数を使う
        k: usize, // 整数の組み合わせの総和が k 以下の組み合わせだけを数え上げる
    }
    let mo = 998244353;
    let mut dp: Vec<Vec<usize>> = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..k {
            for a in 1..=m {
                if j + a <= k {
                    dp[i + 1][j + a] += dp[i][j];
                    dp[i + 1][j + a] %= mo;
                }
            }
        }
    }

    let mut ans = 0;
    for &x in &dp[n] {
        ans += x;
        ans %= mo;
    }
    println!("{}", ans);
}
