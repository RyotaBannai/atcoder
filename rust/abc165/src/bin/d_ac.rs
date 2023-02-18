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
 * Floor Function
 *
 * https://atcoder.jp/contests/abc165/tasks/abc165_d
 *
 * floor(Ax/B)−A×floor(x/B) にx を入れて実験してみると良い.
 * 後ろのfloor ではx/b の結果が1 になってしまうと、前のfloor と計算結果が同じになってしまうため, 全体として0 になる、これが最小.
 *
 * 最大にするためには、後ろのfloor を0 にしつつ前の値を最大にするx であるから、min(n, b-1) とするのが最適.
 *
 *
 */
use library::min;
// #[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        n: f64
    }
    let f = |x: f64| (a * x / b).floor() - a * (x / b).floor();
    println!("{}", f(min!(b - 1., n)));
}
