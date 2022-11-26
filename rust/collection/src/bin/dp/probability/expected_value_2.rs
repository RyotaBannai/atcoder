/**
 * @cpg_dirspec expected_value
 *
 * cpg run -p src/bin/dp/probability/expected_value_2.rs
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
// use collection::{geo_lib::*, utils::read::*};

/**
 * 確率DP 期待値 出た目の和
 *
 * tags: #DP #確率DP
 *
*/

#[fastout]
fn main() {
    input! {
      k: usize
    }

    let ma = k + 1 + 7;
    let mut dp = vec![0.; ma];

    // k-1 から計算することで i 回投げた時にとりうる最大値からちょうど始められる
    for i in (0..k).rev() {
        for k in 1..=6 {
            // 各回の期待値は独立していて、そこに新しい期待値混ぜて再計算するのは正しい
            dp[i] += (dp[i + k] + 1.) / 6.;
        }
    }

    println!("{:.12}", dp[0]);
}
