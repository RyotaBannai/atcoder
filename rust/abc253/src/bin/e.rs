use ac_library_rs::modint::ModInt998244353 as Mint;
use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
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
 * Distance Sequence
 *
 * https://atcoder.jp/contests/abc253/tasks/abc253_e
 *
 * N: 回イテレート
 * M: 最大値
 * K: +- K を新しく加える
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: isize,
    }

    let mut dp = vec![vec![Mint::new(0usize); m + 1]; n + 1]; // 1-index
    for i in 1..=m {
        dp[1][i] = Mint::new(1usize); // 1 個使った時はそれぞれ１通り、m が最大値
    }

    dbg!(&dp);

    for i in 1..n {
        // n 回イテレート
        for j in 1isize..=m as isize {
            if m as isize - j >= k {
                for x in j + k..=m as isize {
                    // dbg!(x);
                    let tmp = dp[i][x as usize];
                    dp[i + 1][j as usize] += tmp;
                }
            }
            if j > k {
                for x in 1..=j - k as isize {
                    let tmp = dp[i][x as usize];
                    dp[i + 1][j as usize] += tmp;
                }
            }
        }
    }

    let mut ans = Mint::new(0usize);
    for x in &dp[n] {
        ans += x;
    }

    println!("{}", ans);
}
