/**
 * @cpg_dirspec tournament
 *
 * cpg run -p src/bin/dp/probability/tournament.rs
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
// use collection::{geo_lib::*, utils::read};

/**
 * 確率DP トーナメント
 *
 * tags: #DP #確率DP
 *
 * https://atcoder.jp/contests/tdpc/tasks/tdpc_tournament
 *
 * k回目の試合後に人iが生き残る確率は、対戦する可能性のあるすべての人と対戦して勝つパターンの確率の合計
 * i がk 連勝するときは、i が k−1 連勝する確率に、対戦相手 j が k−1 連勝して、 i が j に勝つときの確率をかけたもの
 *
*/
// #[fastout]
fn main() {
    input! {
      round: usize // k 回戦目まである
    }
    let n = 1 << round;
    input! {
        e: [f64; n]
    }

    // k 回戦目終わった時点でそれぞれが残っている確率
    // 0 回戦目は全員残っているから確率 1.
    let mut dp = vec![vec![0.; round + 1]; n];
    for i in 0..n {
        dp[i][0] = 1.;
    }

    for k in 1..=round {
        for i in 0..n {
            // ありうる対戦相手の範囲を決める
            // 0 index で 2 回戦目の 4 のありうる対戦相手は、5,6,7
            // 2 回戦目では、５はすでに4に負けているから、この中から除く(for の中の初めの if 文)
            let l = (i >> k) << k;
            let r = l + (1 << k);
            for j in l..r {
                // まず、ありうる対戦相手は [l,r) で絞る
                // 次に、この範囲からさらに絞るには、k 回目の対戦時に自分とありうる対戦相手の、(k-1) 桁目の bit が異なる、という性質を用いる
                // 2 回戦目(k==2)では、4と5は 0100 >> 2 は 0101 >> 2 は同じだから、対戦済みのグループにあると考えられる(１回目の対戦としては異なるため、ありうる対戦相手)
                if i >> (k - 1) & 1 == j >> (k - 1) & 1 {
                    continue;
                }
                let prob_match = dp[j][k - 1];
                // i が j に勝つ確率
                let prob_win = 1. / (1. + 10f64.powf((e[j] - e[i]) / 400.));
                // i に対して、ありうる対戦相手との勝率の総和を求める
                dp[i][k] += prob_match * prob_win;
            }

            dp[i][k] *= dp[i][k - 1];
        }
    }

    for k_probs in &dp {
        println!("{:.7}", k_probs[round]);
    }
}
