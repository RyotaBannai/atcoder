use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, isize>;
// type Set = BTreeSet<isize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
// use library::structure::compress::*;

/**
 * D - Online games
 *
 * https://atcoder.jp/contests/abc221/tasks/abc221_d
 *
 * tags: #貪欲法
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    // 別々に処理しても同じ日付をまとめても問題ない
    // 一番最後のentry は0 になるから、n-1 まで処理する点では変わらない.
    let mut m = Map::new();
    for &(a, b) in &ab {
        *m.entry(a).or_insert(0) += 1; // 一人増えた
        *m.entry(a + b).or_insert(0) -= 1; // 一人減った
    }

    let mut xs = m.iter().collect_vec();
    xs.sort_unstable();
    let len = xs.len();
    let mut ans = vec![0; n + 1]; // 最大n 人までしか増えない.
    let mut count = 0;
    for i in 0..len - 1 {
        count += xs[i].1;
        ans[count as usize] += xs[i + 1].0 - xs[i].0;
    }
    for i in 1..=n {
        print!("{} ", ans[i]);
    }
    println!();
}
