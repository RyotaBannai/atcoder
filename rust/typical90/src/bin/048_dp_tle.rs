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
 * 048 - I will not drop out（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_av
 *
 *
 * 問題数i が 2*10^5
 * 時間が 2*2*10^5
 * だから dp 組んじゃうと O(n^2) でつむ
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [(usize, usize); n]
    }

    let mut dp = vec![vec![0; k + 1]; 2]; // 最大 k 時間使える, それぞれの問題の部分点と満点は二つのコストの違う別の問題と考える

    for (i, (a, b)) in p.iter().enumerate() {
        for j in 0..=k {
            // 問題を解かない選択もある.
            dp[(i + 1) % 2][j] = dp[(i + 1) % 2][j].max(dp[i % 2][j]);

            // 問題を解く
            if j > 0 {
                // 部分点を取るには1分かかる
                dp[(i + 1) % 2][j] =
                    (dp[(i + 1) % 2][j].max(dp[i % 2][j])).max(dp[i % 2][j - 1] + b);
            }

            if j > 1 {
                // コストがover してはならない
                dp[(i + 1) % 2][j] =
                    (dp[(i + 1) % 2][j].max(dp[i % 2][j])).max(dp[i % 2][j - 2] + a);
                // 満点を取るには２分かかる
            }
        }
    }

    let mut ma = 0;
    for x in &dp[n % 2] {
        ma = ma.max(*x);
    }
    println!("{}", ma);
}
