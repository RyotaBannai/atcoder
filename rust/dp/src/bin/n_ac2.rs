use proconio::{fastout, input, marker::Chars};
use std::isize::MAX;
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
 * N - Slimes
 *
 * https://atcoder.jp/contests/dp/tasks/dp_n
 *
 * tags: #range_dp #区間dp　#区間の除去
 *
 *
 * 連結する時は、結局は今までのコスト+その区間の総和(その区間において常に一定) になる
 *
 * 参考
 * https://www.youtube.com/watch?v=YNT_iTpSWGI
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        w: [isize; n]
    }

    let mut sum = vec![vec![0; n + 1]; n + 1];
    for l in 0..n {
        sum[l][l + 1] = w[l];
        for r in l + 2..=n {
            sum[l][r] = sum[l][r - 1] + w[r - 1];
        }
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];

    // width は 2 以上から始める
    // mid を間に挟みたい. そうすると 1 以上の差では、要素１の dp の区間が取れないため問題.
    // 要素１の dp が取れないと、コストがかからない 0 + 0 + sum の計算ができない

    // 最小の組み合わせの幅（width=2）から計算を始めて行って、最後全体の組み合わせ幅（width=n）まで広げた時の最小を求める
    // 最小の幅から求めているため、スライム２つ辺のコストが順に計算できるし、３つ当たりの最小コスト時にはスライム２同士のコストをもとに計算することができる
    for width in 2..=n {
        for l in 0..=n - width {
            let r = l + width;
            let mut mi = MAX;
            dp[l][r] = MAX;
            for m in l + 1..r {
                mi = mi.min(dp[l][m] + dp[m][r] + sum[l][r]);
            }
            dp[l][r] = dp[l][r].min(mi);
        }
    }

    println!("{}", dp[0][n]);
}
