use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Distinct Trio
 *
 * https://atcoder.jp/contests/abc252/tasks/abc252_d
 *
 * tags: #余事象
 */

fn make_tri(x: usize) -> usize {
    if x >= 3 {
        x * (x - 1) * (x - 2) / (3 * 2 * 1)
    } else {
        0
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut v = vec![0; 200_005];
    for x in a {
        v[x] += 1;
    }

    // nC3
    let mut all = make_tri(n); // 組み合わせは先頭から順に選ぶため、i,j,k の条件を満たす
    for x in v {
        // 3 つ選んで 3 つ同じ場合の組み合わせ
        let tri = make_tri(x);
        // 3 つ選んで 2 つ同じ場合の組み合わせ
        let twi = if x >= 2 { (n - x) * x * (x - 1) / 2 } else { 0 };
        // 上記の組み合わせは互いに排反
        all -= tri + twi;
    }

    println!("{}", all);
}
