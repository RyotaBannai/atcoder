use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::utils::dp::add;
use std::usize::MAX;
/**
 * Logical Expression
 *
 * https://atcoder.jp/contests/abc189/tasks/abc189_d
 *
 * tags: #dp #動的計画法 #dp_add
 *
 */

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut dp = vec![vec![MAX; 2]; n + 1]; // i回目で 0: false, 1: true

    dp[0][0] = 1;
    dp[0][1] = 1;
    for i in 0..n {
        for j in 0..=1 {
            if dp[i][j] == MAX {
                continue;
            }

            let b = j == 1;
            let x = dp[i][j];
            if s[i][0] == 'A' {
                add(&mut dp[i + 1][(b && true) as usize], x);
                add(&mut dp[i + 1][(b && false) as usize], x);
            } else {
                add(&mut dp[i + 1][(b || true) as usize], x);
                add(&mut dp[i + 1][(b || false) as usize], x);
            }
        }
    }

    println!("{}", dp[n][1]);
}
