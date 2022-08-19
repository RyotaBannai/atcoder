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
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * O - Matching
 *
 * https://atcoder.jp/contests/dp/tasks/dp_o
 *
 * tags: #dp #bit_dp
 *
 * TLE
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n]
    }

    let mo = 1000000007usize;

    let mut men = vec![vec![]; n];
    for (i, v) in a.iter().enumerate() {
        for (j, &x) in v.iter().enumerate() {
            if x == 1 {
                men[i].push(j);
            }
        }
    }

    let mut dp = vec![vec![0; 1 << n]; n + 1]; // キーは組み合わせの状態 // 値は組み合わせ数

    dp[0][0] = 1;

    // 男性を前から一人づつ使った時、組になった女性の状態を遷移させる
    // 最大n=21 男性 i set
    for (i, s) in men.iter().enumerate() {
        for bit in 0..1 << n {
            // 最大n=21 男性の set から相性の良い組の女性を取り出す
            for x in s {
                // 男性と女性の相性が良い かつ bit の状態 j で女性 x が使われていない場合、新しい組み合わせとして考慮する
                if bit & 1usize << x == 0 {
                    let nbit = bit | 1usize << x;
                    // println!("{}({:#06b}), j {}, x {}", nj, nj, j, x); // debug
                    dp[i + 1][nbit] += dp[i][bit];
                    dp[i + 1][nbit] %= mo;
                }
            }
        }
    }

    println!("{}", &dp[n][(1 << n) - 1]);
}
