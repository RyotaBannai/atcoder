/*
digitnum

https://atcoder.jp/contests/abc238/tasks/abc238_c
 */
use ac_library_rs::modint::ModInt998244353 as Mint;
use proconio::{fastout, input};
use std::cmp::min;
use std::convert::TryInto;

fn f(k: usize) -> usize {
    let mut x = Mint::new(k);
    x *= k + 1;
    x /= 2_usize;
    x.val().try_into().unwrap()
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let m = 998244353_usize;
    let mut ans = 0_usize;
    let mut p10 = 10_usize;
    for _ in 1..=18 {
        let l = p10 / 10;
        let r = min(n, p10 - 1);
        if l <= r {
            ans += f(r - l + 1);
            ans %= m;
        }
        p10 *= 10;
    }

    println!("{}", ans);
}
