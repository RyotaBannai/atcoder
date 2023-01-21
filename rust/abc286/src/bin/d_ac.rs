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
 * Money in Hand
 *
 * https://atcoder.jp/contests/abc286/tasks/abc286_d
 *
 * tags: #dp #動的計画法
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n]
    }

    let ma = x + 2;
    let mut dp = vec![vec![None; ma]; n + 1];
    dp[0][0] = Some(true);
    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..=x {
            for k in 0..=b {
                if j + k * a > x {
                    break;
                }
                if dp[i][j].is_some() {
                    dp[i + 1][j + k * a] = Some(true);
                }
            }
        }
    }
    // println!("{:?}", &dp);
    let ans = dp[n][x];
    if ans.is_some() {
        println!("Yes");
    } else {
        println!("No");
    }
}
