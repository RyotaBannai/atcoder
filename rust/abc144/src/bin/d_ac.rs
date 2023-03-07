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
 * Water Bottle
 *
 * https://atcoder.jp/contests/abc144/tasks/abc144_d
 *
 * tags: #math #角度 #atan
 *
 * ボトルの水が半分より大きいか小さいかで場合分する.
 *
 * 参考
 * https://twitter.com/kyopro_friends/status/1188451564588957697
 *
 */
use library::geometry::vector::prelude::*;
// #[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    }
    let s = x / a; // 面積
    if a * b / 2. > s {
        // 半分より小さい
        let c = 2. * s / b;
        println!("{}", to_deg((b / c).atan()));
    } else {
        // 半分より大きい
        let t = a * b - s;
        let c = (2. * t) / a;
        println!("{}", to_deg((c / a).atan()));
    }
}
