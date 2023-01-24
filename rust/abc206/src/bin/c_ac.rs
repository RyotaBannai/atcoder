use itertools::Itertools;
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
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Swappable
 *
 * https://atcoder.jp/contests/abc206/tasks/abc206_c
 *
 * 事前に同じ整数がどれだけあるか計算する
 * 整数i はi 以外の整数と組み合わせられるから、配列A がN 個の元からなるとすると
 * (N-i)*i の組み合わせができる.
 *
 * また同じ組み合わせをカウントしないようにするためには、
 * 組み合わせの処理をした整数の個数v をN から順に引いていくと良い.
 *
 */

// #[fastout]
fn main() {
    input! {
        mut n: usize,
        a: [usize; n]
    }
    let mut m = Map::new();
    for x in a {
        *m.entry(x).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (_, v) in m.iter() {
        ans += (n - v) * v;
        n -= v;
    }
    println!("{}", ans);
}
