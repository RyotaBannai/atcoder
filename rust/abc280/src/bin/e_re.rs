use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: usize,
    }

    let zero = Mint::new(0usize);
    let hund = Mint::new(100usize);
    // どれくらい i 回目の思考で、j 体力が残っているときの期待値
    let mut dp = vec![vec![zero; n + 1]; n + 1];
    dp[0][n] = hund; // 0 回試行した時にはn 残っている確率 1
    for i in 0..n - 1 {
        // j は残りの体力
        // value は期待値
        for j in 0..=n {
            if j == 0 {
                dp[i + 1][0] = dp[i][0];
            } else if j == 1 && dp[i][j] != zero {
                let mut tmp = dp[i][j] * (i + 1);
                if i >= 1 {
                    tmp /= i;
                }
                dp[i + 1][0] += tmp;
            } else if j >= 2 && dp[i][j] != zero {
                // 1 減らす
                let mut tmp1 = dp[i][j] * (i + 1) * (100 - p);
                tmp1 /= hund;
                if i >= 1 {
                    tmp1 /= i;
                }
                dp[i + 1][j - 1] += tmp1;

                let mut tmp2 = dp[i][j] * (i + 1) * p;
                tmp2 /= hund;
                if i >= 1 {
                    tmp2 /= i;
                }
                // 2 減らす
                dp[i + 1][j - 2] += tmp2;
            }
        }
    }
    // println!("{:?}", &dp);
    let mut ans = dp[n - 1][0] + dp[n - 1][1] / (n - 1) * n;
    ans /= hund;
    println!("{}", ans);
}
