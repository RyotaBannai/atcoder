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
 * D - Index × A(Not Continuous ver.)
 *
 * https://atcoder.jp/contests/abc267/tasks/abc267_d
 *
 * i 回目まで使うかどうかの２択を選んで、m 個使ったかどうかを管理する
 * m 回使うとき:=m-1まで選択できている状態（:=i-1,j=m-1 に値が入っていることで確認できる）で、m-1 目はあらゆる選び方を考慮した上で最適な操作をしていると考える.  ポイントはちょうど m-1 使っているということ
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n]
    }

    // n 番目の元をチェックした時, m 個使ったことになる時の数値
    // dp[nth までチェック][m 使った]
    let mut dp = vec![vec![None; m + 1]; n + 1];
    dp[0][0] = Some(0);

    for i in 0..n {
        for j in 0..=m {
            dp[i + 1][j] = dp[i][j]; // デフォルト // ちょうど上から流した（使わない場合として与える）

            if j > 0 {
                if let Some(x) = dp[i][j - 1] {
                    dp[i + 1][j] = dp[i][j].max(Some(x + a[i] * j as isize));
                    // 使わない場合（上から流しただけ）と斜め左からちょうどn-1 回使って n にする時の数を比較. let Some でそもそも、n-1 回目すら計算していない index を参照しているときは弾いている(例えば、i=1,j=2 で i=0,j=1 を参照したい時に、i=1 で初めて一回目を使うかどうか見ているから、i=0,j=1 は 1 個も使っていない場合であり、j=2 として、２回目を使うかどうかの判定するのは不正)
                    // (i+1).min(m) として弾いてもいい
                }
            }
        }
    }

    println!("{}", dp[n][m].unwrap());
}
