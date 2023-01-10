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
use library::*;
use std::usize::MAX;
/**
 * D - Strange Lunchbox
 *
 * https://atcoder.jp/contests/abc219/tasks/abc219_d
 *
 * tags: #DP #動的計画法 #attention
 *
 * 最後ちょうど x,y になる回も回す.
 * 最後のitem を使わない場合（最後を使わなくても最適解になる場合）
 * 前回 i-1 のちょうどx,y になるcount が最適解になるから、
 * ここまで回さないとWA
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize,usize); n]
    }

    let mut dp = vec![vec![vec![MAX; y + 1]; x + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..=x {
            for k in 0..=y {
                if dp[i][j][k] != MAX {
                    let c = dp[i][j][k];
                    // 使わない
                    chmin!(dp[i + 1][j][k], c);
                    //使う
                    let (nj, nk) = (x.min(j + a), y.min(k + b));
                    chmin!(dp[i + 1][nj][nk], c + 1);
                }
            }
        }
    }
    if dp[n][x][y] == MAX {
        println!("-1");
    } else {
        println!("{}", dp[n][x][y]);
    }
}
