// use ac_library_rs::modint::ModInt998244353 as Mint;
use proconio::{fastout, input, marker::Chars};
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
use std::cmp::{max, min};

/**
 * 1111gal password
 *
 * https://atcoder.jp/contests/abc242/tasks/abc242_c
 *
 * ・deg が 1 桁から考えて、計算済みの結果を用いて 2 桁目以降を計算
 *
*/

#[fastout]
fn main() {
    input! { n: usize }
    let mo = 998244353;

    let mut v = vec![vec![0; 10]; 2]; // n+1 用意して、deg-1 を参照してもよい
    for i in 1..=9 {
        v[0][i] = 1;
    }

    for deg in 1..n {
        let x = (deg + 1) % 2;
        for j in 1..=9 {
            let mut sum = 0;
            for k in max(j - 1, 1)..=min(j + 1, 9) {
                // dbg!(k);
                sum += v[x][k as usize];
                sum %= mo;
            }
            v[deg % 2][j as usize] = sum;
            // std::process::exit(0);
        }
    }
    let mut ans = 0;
    for x in &v[(n + 1) % 2] {
        ans += x;
        ans %= mo;
    }
    // dbg!(&v);
    println!("{}", ans);
}
