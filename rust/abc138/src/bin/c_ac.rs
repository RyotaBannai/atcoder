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

/**
 *
 * https://atcoder.jp/contests/abc138/tasks/abc138_c
 *
 * 数a,b を足して２で割る時に、大きい数を先に使ってしまうと、それ以降の計算で毎回1/2 されてしまうから多く数が減ってしまう.
 * 1000/2 と2/2 では減り方が違う.
 * そのため、小さい数を先に使って小さい数に対して最後まで 1/2 を適用することが最適解になる.
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    }

    a.sort_unstable();
    let mut t = a[0] as f64;
    for x in a.into_iter().skip(1) {
        t = (t + x as f64) / 2.;
    }
    println!("{}", t);
}
