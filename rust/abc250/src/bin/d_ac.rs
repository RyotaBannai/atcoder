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
use library::number::prime;

/**
 * 250-like Number
 *
 * https://atcoder.jp/contests/abc250/tasks/abc250_d
 *
 * tags: #素数
 *
 * q^3 部分を計算すると overflow する可能性があるから、
 * n の分母に持ってくることで回避する.
 * 全素数のペアの組み合わせチェックしても少ないから間に合う.
 *
 *
 * 同様なテクニック
 * 典型90 Multiplication 085（★4）typical90/src/bin/085_ac.rs
 *
 */

// #[fastout]
fn main() {
    input! {
        n: f64
    }

    if n <= 5. {
        println!("0");
        return;
    }

    let (_, ps) = prime(n.cbrt() as usize);
    let mut ans = 0;
    for i in 0..ps.len() - 1 {
        for j in i + 1..ps.len() {
            if ps[j] as f64 * ps[j] as f64 <= (n / ps[i] as f64 / ps[j] as f64) {
                ans += 1;
            } else {
                break;
            }
        }
    }
    println!("{}", ans);
}
