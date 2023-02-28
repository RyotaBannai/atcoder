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

use library::chmin;
use std::usize::MAX;
/**
 * Crested Ibis vs Monster
 *
 * https://atcoder.jp/contests/abc153/tasks/abc153_e
 *
 * tags: #DP #動的計画法
 *
 * h<=10^3
 * n<=10^4
 *
 * だから、あるh ごとに魔法を10^4 使ってある状態s から状態s+1 へ遷移するときに必要な計算量はO(nh)=O(10^7) に収まる
 */
// #[fastout]
fn main() {
    input! {
        h: usize,
        n: usize,
        ab: [(usize, usize); n]
    }

    // i 番目の魔法を使えるだけ使った時に、h 体力が残っている時の最小の魔力
    let mut dp = vec![MAX; h + 1];
    dp[h] = 0; // h 残っている時には魔力を１度も使っていない
    for i in (0..=h).rev() {
        if dp[i] == MAX {
            continue;
        }
        for &(a, b) in ab.iter() {
            chmin!(dp[i.saturating_sub(a)], dp[i] + b);
        }
    }
    println!("{}", dp[0]);
}
