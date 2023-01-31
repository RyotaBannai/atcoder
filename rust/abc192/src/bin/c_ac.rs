use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
// max, min,
// Ordering::{Equal, Greater, Less},
// };
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::utils::conv::*;

/**
 * Kaprekar Number
 *
 * https://atcoder.jp/contests/abc192/tasks/abc192_c
 *
 *
 */

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let g1 = |xs: &[usize]| xs.iter().cloned().sorted().collect_vec();
    let g2 = |xs: &[usize]| xs.iter().cloned().sorted_by(|a, b| b.cmp(a)).collect_vec();
    let f = |xs: &[usize]| build_i(&g1(xs)) - build_i(&g2(xs)); // build_i はindex-0 が１の位
    let mut xs = deassemble_i(n);
    for _ in 0..k {
        xs = deassemble_i(f(&xs));
    }

    if xs.is_empty() {
        println!("0");
    }

    for x in xs {
        print!("{}", x);
    }
}
