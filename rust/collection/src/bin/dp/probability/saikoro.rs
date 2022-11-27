/**
 * @cpg_dirspec saikoro
 *
 * cpg run -p src/bin/dp/probability/saikoro.rs
 */
use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
// use library::{geo_lib::*, utils::read::*};

/**
 * 確率DP 出た目の和
 *
 * tags: #DP #確率DP
 *
 * 1 から６までの整数が等確率に出るサイコロが１つある。このサイコロをN回振るとき、でための数の和がK以上になる確率を求めよ
 *
*/
#[fastout]
fn main() {
    input! {
      n: usize,
      k: usize
    }

    let mut dp = vec![vec![0f64; 6 * (n + 1) + 1]; n + 1];
    dp[0][0] = 1.;

    for i in 0..n {
        for j in 0..6 * n {
            // i+1回目に整数mの目が出た
            for m in 1..=6 {
                // i 回目の確率をi+1目に加える. i 回目の確率の 6. 分の１が加わる
                dp[i + 1][j + m] += dp[i][j] / 6.;
            }
        }
    }

    let mut total = 0.;
    for (i, x) in dp[n].iter().enumerate() {
        if i >= k {
            total += x;
        }
    }

    println!("{}", total);
}
