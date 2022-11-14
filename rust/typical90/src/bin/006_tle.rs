use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
 * Smallest Subsequence（★5）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_f
 *
 * TLE
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }
    let mut dp = vec![vec!["".to_string(); k]; 2]; // 長さ n まで確保, ひとつ前の状態だけ管理
    let mut i = 0;
    for (_, c) in s.iter().enumerate() {
        for j in 0..k {
            // let prev = i;
            // let now = i + 1;
            let prev = i % 2;
            let now = (i + 1) % 2;
            // 今回も文字 c を考慮しない場合
            if j == 0 {
                if dp[prev][0] == "" {
                    dp[now][0] = c.to_string();
                } else {
                    dp[now][0] = dp[prev][0].clone().min(c.to_string());
                }
                continue;
            }

            if dp[now][j] == "" {
                dp[now][j] = dp[prev][j].clone(); // 前の文字列をそのまま引き続き
            } else {
                if dp[prev][j] != "" {
                    dp[now][j] = dp[now][j].clone().min(dp[prev][j].clone());
                }
            }

            // 考慮する場合、文字を後ろに加える.
            // ベースの文字列がないなら処理しない
            // println!("{:?}", &dp[prev]);
            if dp[prev][j - 1] != "" {
                let m = format!("{}{}", dp[prev][j - 1], c);
                if dp[now][j] == "" {
                    dp[now][j] = m;
                } else {
                    dp[now][j] = dp[now][j].clone().min(m);
                }
            }
        }
        i += 1;
    }
    // println!("{:?}", &dp);

    println!("{}", dp[i % 2][k - 1]);
}
