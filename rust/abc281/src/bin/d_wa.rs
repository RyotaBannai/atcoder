use itertools::Itertools;
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
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::*;
/**
 * D - Max Multiple
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n]
    }

    // [[商; 余り]; n 回目使った]
    let mut dp = vec![vec![vec![None; d + 1]; k + 1]; 2]; // あまり、尚
    dp[0][0][0] = Some(0); // 初回は　余りも商も 0, 使った回数も 0

    for i in 0..n {
        for l in 0..k {
            for j in 0..d {
                // j はその時の余り
                if let Some(z) = dp[i % 2][l][j] {
                    let x = a[i]; // i 番目の要素を使う.

                    // x を使う場合
                    let p = ((j + x) / d) + z;
                    let q = (j + x) % d;
                    chmax!(dp[(i + 1) % 2][l + 1][q], Some(p)); // 商は加えていく

                    // x を使わない
                    chmax!(dp[(i + 1) % 2][l][j], Some(z));
                }
            }
        }
    }

    // println!("{:?}", &dp[n][k]);
    // for xs in &dp {
    // println!("{:?}", &xs);
    // }

    if let Some(a) = dp[n % 2][k][0] {
        println!("{}", a * d);
    } else {
        println!("-1");
    }
}
