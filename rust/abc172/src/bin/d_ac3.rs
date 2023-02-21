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
// use std::collections::{BinaryHeap, VecDeque};

use library::number::*;

/**
 * Sum of Divisors
 *
 * https://atcoder.jp/contests/abc172/tasks/abc172_d
 *
 * tags: #約数 #math
 *
 *
 * 割った時の個数が同じになる整数をまとめて計算する方法
 *
 * 初めに約数を求めて、i をインクリメントしながら次の約数と割った時の個数が同じになる位置までまとめて計算することで計算量を減らす.
 *
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut ans = 0;
    let asum = |k: usize| k * (k + 1) / 2; // 自然数の和の公式
    let sum = |a: usize, l: usize| (l - a + 1) * (a + l) / 2; // 等差数列の和
    let divs = divisor(n);
    let mut cur = 0;
    let mut i = 1;
    while i <= n {
        if (n / i) == (n / divs[cur]) {
            ans += sum(i, divs[cur]) * asum(n / i);
            i = divs[cur];
            cur += 1;
        } else {
            ans += sum(i, i) * asum(n / i);
        }
        i += 1;
    }
    println!("{}", ans);
}
