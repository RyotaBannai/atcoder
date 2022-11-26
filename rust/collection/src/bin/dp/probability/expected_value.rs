/**
 * @cpg_dirspec expected_value
 *
 * cpg run -p src/bin/dp/probability/expected_value.rs
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
    let mut dp = vec![vec![0usize; ma]; k + 1];
    dp[0][0] = 1;

    // k 回 1 が出る場合が、サイコロを降る最大の回数だからそれを最後とする
    for i in 0..k {
        // k より小さい場合だけ、i 回目でチェック
        // この回に k 以上になった場合を後に集計
        for j in 0..k {
            for m in 1..=6 {
                dp[i + 1][j + m] += dp[i][j];
            }
        }
    }

    let counts = dp
        .iter()
        .skip(1) // 0 回目を取り除く
        .map(|xs| xs.iter().skip(k).collect::<Vec<_>>()) // k より小さい場合を取り除く
        // .inspect(|xs| println!("{:?}", xs))
        .map(|xs| xs.into_iter().sum::<usize>()) // k 以上になる回数を集計
        // .inspect(|x| println!("{:?}", x))
        .collect::<Vec<_>>();

    // 期待値を計算
    // x 回の組み合わせが i 回目に k 以上になることを考えるとき、
    // その i 回目に期待値は、x * i / 6^i
    let mut sum = 0.;
    for (i, x) in counts.iter().enumerate() {
        sum += (x * (i + 1)) as f64 / (6f64).powf((i + 1) as f64)
    }

    println!("{:.12}", sum);
}
