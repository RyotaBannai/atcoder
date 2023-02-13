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
 * Ubiquity
 *
 * https://atcoder.jp/contests/abc178/tasks/abc178_c
 *
 * tags: #dp #動的計画法 #状態
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mo = 1000000007;
    let mut dp = vec![vec![0; 4]; n + 1];
    for i in 0..=9 {
        if i == 0 {
            dp[1][1] += 1;
        } else if i == 9 {
            dp[1][2] += 1;
        } else {
            dp[1][0] += 1;
        }
    }
    for i in 1..n {
        for j in 0..=9 {
            for l in 0..=3 {
                // 0: どっちも入ってない
                // 1: 0 が入ってる
                // 2: 9 が入ってる
                // 3: どっちも入ってる

                // 状態が変化する場合だけ考えて、それ以外の状態に変化がないケースはelse
                if l == 3 || l == 1 && j == 9 || l == 2 && j == 0 {
                    dp[i + 1][3] += dp[i][l];
                    dp[i + 1][3] %= mo;
                } else if l == 0 && j == 0 {
                    dp[i + 1][1] += dp[i][l];
                    dp[i + 1][1] %= mo;
                } else if l == 0 && j == 9 {
                    dp[i + 1][2] += dp[i][l];
                    dp[i + 1][2] %= mo;
                } else {
                    dp[i + 1][l] += dp[i][l];
                    dp[i + 1][l] %= mo;
                }
            }
        }
    }

    println!("{}", dp[n][3]);
}
