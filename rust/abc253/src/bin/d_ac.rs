use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::number::*;

/**
 * FizzBuzz Sum Hard
 *
 * tags: #等差数列 #総和 #最小公倍数 #lcm
 *
 * https://atcoder.jp/contests/abc253/tasks/abc253_d
 *
 * 参考
 * https://www.suzu6.net/posts/292-rust-num-crates/#%E6%9C%80%E5%A4%A7%E5%85%AC%E7%B4%84%E6%95%B0%E3%81%A8%E6%9C%80%E5%B0%8F%E5%85%AC%E5%80%8D%E6%95%B0
 */

#[fastout]
fn main() {
    input! {
        n:isize,
        a:isize,
        b:isize
    };

    let i = n / a;
    let sum_1 = (a + (i * a)) * i / 2;

    let j = n / b;
    let sum_2 = (b + (j * b)) * j / 2;

    let lcm = lcm(vec![a, b]);
    let k = n / lcm;
    let sum_3 = (lcm + (k * lcm)) * k / 2;

    let total = (1 + n) * n / 2;

    println!("{}", total - sum_1 - sum_2 + sum_3);
}
