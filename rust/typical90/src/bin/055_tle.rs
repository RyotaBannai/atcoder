use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
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
 * 055 - Select 5（★2）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bc
 *
 * TLE
 * p <=100,000
 * くらいの制約であれば通りそう
 */

#[fastout]
fn main() {
    input! {
        n: usize, // <=100
        p: usize,
        q: usize,
        xs: [usize; n]
    }

    let mut dp = vec![Map::new(); 6];
    dp[0].insert(1, 1);
    for x in xs {
        for i in (0..5).rev() {
            for (k, v) in dp[i].clone() {
                let m = k * x % p;
                if let Some(y) = dp[i + 1].get_mut(&m) {
                    *y += v;
                } else {
                    dp[i + 1].insert(m, v);
                }
            }
        }
    }

    println!("{}", dp[5].get(&q).unwrap_or(&0));
}
