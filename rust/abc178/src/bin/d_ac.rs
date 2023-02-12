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
 * Redistribution
 *
 * https://atcoder.jp/contests/abc178/tasks/abc178_d
 *
 * tags: #dp #動的計画法
 *
 *
 */
use library::utils::dp::add;

// #[fastout]
fn main() {
    input! {
        s: usize,
    }
    use std::usize::MAX;
    let mut dp = vec![MAX; s + 1];
    dp[0] = 1;
    for j in 3..=s {
        for k in 3..=s {
            if j < k {
                // もし作りたい数値が k ステップより小さいならpass
                continue;
            }
            let x = dp[j - k];
            if x == MAX {
                // 作れない.
                continue;
            }
            add(&mut dp[j], x);
            dp[j] %= 1_000_000_007;
        }
    }
    if dp[s] == MAX {
        println!("0");
    } else {
        println!("{}", dp[s]);
    }
}
