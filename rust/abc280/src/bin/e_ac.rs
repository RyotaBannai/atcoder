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

/**
 * E - Critical Hit
 *
 * https://atcoder.jp/contests/abc280/tasks/abc280_e
 *
 * tags: #DP #確率DP #期待値
 *
 * 期待値の計算は、他の期待値から「集めるように計算」する：
 *
 * ある期待値A（複数を想定）から新たな期待値B を作るときは、A にB になるための期待値を先に計算して（先に確率をかけて）その結果に1 を加える.
 *  -> 期待値B では、全ての状態（期待値A）からしか B が作られないことがわかっている. つまりそれらの状態は期待値B になるために考えられうる全ての確率を含むから、
 *   　「計算した結果に1 を加える」.
 *
 * 他の実装例：collection/src/bin/dp/probability/expected_value.rs
 *
 * 「体力が0 になる時の期待値」だけど、「N 減らせる時」と考えた方がやりやすい.
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        p: usize,
    }

    let zero = Mint::new(0usize);
    let mut dp = vec![zero; n + 3];
    for i in (0..n).rev() {
        let a = (dp[i + 1] + 100) * (100 - p) / 100;
        let b = (dp[i + 2] + 100) * p / 100;
        dp[i] += a + b;

        // dbg!(i);
        // dbg!(a);
        // dbg!(b);
        // println!("{:?}", &dp);
    }
    println!("{}", dp[0] / 100);
}
