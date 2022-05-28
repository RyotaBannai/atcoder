use ac_library_rs::modint::ModInt998244353 as Mint;
use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
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
 * Distance Sequence
 *
 * https://atcoder.jp/contests/abc253/tasks/abc253_e
 *
 * tags: #DP #累積和 #以外範囲
 *
 * N: 桁分イテレート
 * M: 最大値
 * K: +- K を新しく加える
 *
 *
 * 問題点：
 * ・各 index（j の loop）に対して、 1<=x<=j-k, j+k<=x<=M を回していたため、N^2 になってしまう
 * → 累積和で n-1 の和を計算する
 *
 *
 * 参考
 * https://www.youtube.com/watch?v=Xy1o-33wDIk
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut dp = vec![vec![Mint::new(0usize); m + 1]; n + 1]; // 1-index
    for i in 1..=m {
        dp[1][i] = Mint::new(1usize); // 1 個使った時はそれぞれ１通り、m が最大値
    }

    for i in 1..n {
        // n 桁分イテレート

        let mut cum_sum = dp[i].clone();
        for j in 1..m {
            let tmp = cum_sum[j];
            cum_sum[j + 1] += tmp;
        }

        // それぞれの j について、(二桁目について、一桁目のパターン数を累積和から) 求める
        for j in 1..=m {
            // 0 === j-k ●== j ==● j+k ==● m
            // j.checked_sub(k).unwrap_or(0) と等価！
            dp[i + 1][j] += cum_sum[m];
            if k != 0 {
                dp[i + 1][j] -= cum_sum[(min(m, j + k - 1))] - cum_sum[j.saturating_sub(k)];
            }
        }
    }

    let mut ans = Mint::new(0usize);
    for x in &dp[n] {
        ans += x;
    }

    println!("{}", ans);
}
