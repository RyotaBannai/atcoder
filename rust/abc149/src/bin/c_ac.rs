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
use abc149::nt_lib::*;

/**
 * C - Next Prime
 *
 * https://atcoder.jp/contests/abc149/tasks/abc149_c
 *
 * tags: #prime #素数判定
 *
 * 先に篩にかけて素数表を作成しておく
 *
 */
#[fastout]
fn main() {
    input! {
        x: usize
    }
    let primes = prime(2 * x);
    for x in x..2 * x {
        if primes[x] {
            println!("{}", x);
            return;
        }
    }
}
