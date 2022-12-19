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
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Dice Sum
 *
 * https://atcoder.jp/contests/abc248/tasks/abc248_c
 *
 * DP と順列で無理やり求める. TLE
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize, // k 個の整数の組み合わせ
        m: usize, // m 以下の非負整数を使う
        k: usize, // 整数の組み合わせの総和が k 以下の組み合わせだけを数え上げる
    }

    let mut dp: Vec<Vec<Vec<(Map, usize, usize)>>> = vec![vec![vec![]; k + 1]; n + 1];
    dp[0][0] = vec![(Map::new(), 0, 0)];

    for a in 1..=m {
        for i in 0..n {
            for j in 0..k {
                // 現在の合計値に対して a を加えることをしたい.
                for (mut m, sum, c) in dp[i][j].clone() {
                    if sum + a <= k {
                        *m.entry(a).or_insert(0) += 1;
                        dp[i + 1][j + a].push((m, sum + a, c + 1));
                    }
                }
            }
        }
    }

    // for xs in &dp {
    //     println!("{:?}", xs);
    // }
    // [[({}, 0)], [], [], [], []]
    // [[], [({1: 1}, 1)], [({2: 1}, 2)], [({3: 1}, 3)], []]
    // [[], [], [({1: 2}, 2)], [({1: 1, 2: 1}, 3)], [({2: 2}, 4), ({1: 1, 3: 1}, 4)]]

    let mut ans = Mint::new(0usize);
    for v in &dp[n] {
        for (m, _, c) in v {
            // 重複を許した順列の計算
            let mut a = Mint::new(1usize);
            for i in 2..=*c {
                a *= i;
            }
            for &val in m.values() {
                if val > 1 {
                    for i in 2..=val {
                        a /= i;
                    }
                }
            }
            ans += a;
        }
    }

    println!("{}", ans);
}
