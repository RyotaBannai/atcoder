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
use std::usize::MAX;

/**
 * D - FG operation
 *
 * https://atcoder.jp/contests/abc220/tasks/abc220_d
 *
 * tags: #動的計画法 #dp
 *
 * 結局は先頭は 0~9 の整数に収束するから、
 * その時点での組み合わせ数から、i 番目の先頭との
 * f g の操作による結果の 新しい 0~9 に組み合わせ数を加算していくだけで良い.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let f = |a: usize, b: usize| (a + b) % 10;
    let g = |a: usize, b: usize| (a * b) % 10;

    let mo = 998244353;
    let mut dp = vec![vec![MAX; 10]; n + 1];
    dp[0][a[0]] = 1;

    for i in 1..n {
        for j in 0..10usize {
            if dp[i - 1][j] == MAX {
                continue;
            }
            // for op in &[f, g] { .. } が Rust 14.2 (AtCoder でのversion)が使えない.
            let mut op = |op: fn(usize, usize) -> usize| {
                if dp[i][op(j, a[i])] == MAX {
                    dp[i][op(j, a[i])] = dp[i - 1][j];
                } else {
                    dp[i][op(j, a[i])] += dp[i - 1][j];
                }
                dp[i][op(j, a[i])] %= mo;
            };
            op(f);
            op(g);
        }
    }

    for i in 0..10 {
        let x = dp[n - 1][i];
        if x == MAX {
            println!("0");
        } else {
            println!("{}", x);
        }
    }
}
