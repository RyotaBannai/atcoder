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
 * tags: #DP #動的計画法 #累積和
 *
 * 加えられる区間（1~=M 区間の和）をあらかじめ累積和で求めておくと、
 * さらに高速に求まる.
 * 加える範囲が単調の場合は、こういうように累積和を用いる
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
        let mut sum = vec![0; k + 1]; // 最大k までの和
        for l in 0..k {
            sum[l + 1] = sum[l] + dp[i][l];
            sum[l + 1] %= mo;
        }
        // println!("{:?}", &sum);
        for j in 0..=k {
            // もらうDP (他の和から j を作る場合を考える)
            // 本当は a=1~=M の間の整数からj を作れる場合を考えたいから、j-1~=j-M のそれぞれの組み合わせを j にかき集めることをしたい.
            // 下限の j-M を含めたいから、saturating_sub するときは、m+1 として、さらに１つ小さい位置を見つけることをする.
            // ただし、累積和を使っていて1-index となるため、indexing する値を１小さくする(jから1つ近い位置にする)
            // 実質a=0~=m-1 みる. r=0, l=m
            let l = j.saturating_sub(m);
            let r = j.saturating_sub(0);
            dp[i + 1][j] += (sum[r] + mo) - sum[l];
            dp[i + 1][j] %= mo;
        }

        // println!("{:?}", &dp);
    }

    let mut ans = 0;
    for &x in &dp[n] {
        ans += x;
        ans %= mo;
    }
    println!("{}", ans);
}
