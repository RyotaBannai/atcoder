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
 * 100 to 105
 *
 * https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_c
 *
 * tags: #dp #動的計画法
 *
 * x<=10^5
 * 各値段は 100<N だから最大でも 1000 個しか買えない
 * 商品の種類は 5 だから、計算量 O(5*1000)
 *
 */
// #[fastout]
fn main() {
    input! {
        x: usize,
    }

    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for i in 100..=x {
        for j in 100..=105 {
            if i < j {
                continue;
            }
            if dp[i - j] {
                dp[i] = true;
            }
        }
    }
    if dp[x] {
        println!("1");
    } else {
        println!("0");
    }
}
