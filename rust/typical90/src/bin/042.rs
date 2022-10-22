use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
use ac_library_rs::modint::ModInt1000000007 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

#[fastout]
fn main() {
    input! {
        k: usize
    }

    if k % 9 != 0 {
        println!("0");
        return;
    }

    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for x in 1..=9 {
        // j 各総和になるパターン数
        for j in 1..=k {
            if j < x {
                continue;
            }
            dp[j] += dp[j - x]; // 一つ引いた部分を見るだけで良い
        }
    }

    // for xs in &dp {
    //     println!("{:?}", xs);
    // }
    // println!("ptn {}", dp[9][k]);

    let mut ans = Mint::new::<usize>(1);
    for i in 1..=dp[k] {
        ans *= i;
    }
    println!("{}", dp[k]);
}
