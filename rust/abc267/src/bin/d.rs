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
 * dp っぽい
 * けど j が本当に使っているかどうかの判定ができないから、a[i] * j とした時に不正になる
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
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..n {
        dp[i + 1] = dp[i].clone();
        for j in 0..n.min(m) {
            // 使う
            let tmp = dp[i][j] + a[i] * (j + 1) as isize;

            if dp[i][j] < tmp && dp[i][j + 1] < tmp {
                dp[i + 1][j + 1] = tmp; // i 番目の元
            }
        }
    }

    println!("{}", dp[n][m]);
    // println!("{:?}", dp);
}
