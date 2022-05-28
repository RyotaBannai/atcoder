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

/**
 * FizzBuzz Sum Hard
 *
 * tags: #等差数列 #総和
 *
 * https://atcoder.jp/contests/abc253/tasks/abc253_d
 *
 */

#[fastout]
fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize
    };

    let i = n / a;
    let sum_1 = (a + (i * a)) * i / 2;

    let j = n / b;
    let sum_2 = (b + (j * b)) * j / 2;

    let lcm = num::integer::lcm(a, b);
    let k = n / lcm;
    let sum_3 = (lcm + (k * lcm)) * k / 2;

    let total = (1 + n) * n / 2;

    println!("{}", total - sum_1 - sum_2 + sum_3);
}
