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
 * Billiards
 *
 * https://atcoder.jp/contests/abc183/tasks/abc183_b
 *
 * tags: #式変形
 *
 * 跳ね返るx 座標をx とおいて、傾きを負にして式変形する
 *
 */
// #[fastout]
fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    }

    println!("{}", (gy * sx + sy * gx) as f64 / (sy + gy) as f64);
}
