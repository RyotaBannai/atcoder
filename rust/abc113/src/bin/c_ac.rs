use proconio::{fastout, input, marker::Chars};
use superslice::Ext;
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - ID
 *
 * https://atcoder.jp/contests/abc113/tasks/abc113_c
 *
 * tags: #座標圧縮 #compress
 *
 * ゼロ埋め:
 * https://minerva.mamansoft.net/Notes/Rust%E3%81%A7%E3%82%BC%E3%83%AD%E5%9F%8B%E3%82%81%E3%80%81%E7%A9%BA%E7%99%BD%E5%9F%8B%E3%82%81format
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        py: [(usize, usize); m]
    }
    let mut v = vec![vec![]; n];
    for (p, y) in &py {
        v[p - 1].push(y);
    }

    for xs in v.iter_mut() {
        xs.sort_unstable();
    }

    for (p, y) in &py {
        let pos = v[p - 1].lower_bound(&y);
        println!("{:0>6}{:0>6}", p, pos + 1);
    }
}
