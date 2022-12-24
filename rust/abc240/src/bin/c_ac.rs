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
 * C - Jumping Takahashi
 *
 * https://atcoder.jp/contests/abc240/tasks/abc240_c
 *
 * tags: #動的計画法 #dp
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n]
    }

    let mut dp = vec![vec![None; x + 1]; n + 1];
    dp[0][0] = Some(1);
    for i in 0..n {
        for j in 0..x {
            // 配る
            if let Some(m) = dp[i][j] {
                for k in &[ab[i].0, ab[i].1] {
                    if j + k <= x {
                        let l = dp[i + 1][j + k].unwrap_or(0);
                        dp[i + 1][j + k] = Some(l + m);
                    }
                }
            }
        }
    }

    if dp[n][x].is_some() {
        println!("Yes");
    } else {
        println!("No");
    }
}
