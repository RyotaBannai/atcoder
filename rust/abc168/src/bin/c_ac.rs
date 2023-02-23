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

use library::geometry::vector::prelude::*;

/**
 * C - : (Colon)
 *
 * https://atcoder.jp/contests/abc168/tasks/abc168_c
 *
 * tags: #math #geometry #幾何学 #余弦定理 #deg_to_rad #時間 #鋭角 #鈍角 #三角関数 #trigonometry
 *
 * - 「時針」は「分針」が動いている時も動くことを考慮する. 「時針」 1 つ分が30 だから m/60 * 30 で求める.
 * - 鈍角の時は辺は鋭角側にできるから、角度を変換する
 */

// #[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64
    }
    let theh = 360. / 12. * h + m / 60. * 30.;
    let them = 360. / 60. * m;
    let mut dif = (theh - them).abs();
    if dif > 180. {
        dif = 360. - dif; // 鈍角から鋭角
    }
    let ans2 = a * a + b * b - 2. * a * b * to_rad(dif).cos(); // 余弦定理のθの定義域 0<=θ<=180
    println!("{:.15}", ans2.sqrt());
}
