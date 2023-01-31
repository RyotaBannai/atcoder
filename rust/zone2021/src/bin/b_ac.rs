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
 * 友好の印
 *
 * https://atcoder.jp/contests/zone2021/tasks/zone2021_b
 *
 * tags: #一次方程式
 *
 * 一番傾きが小さい時の切片
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        d: f64,
        h: f64,
        xy: [(f64, f64); n]
    }
    let mut a = h / d;
    for (x, y) in xy {
        let na = (h - y) / (d - x); // 新しい傾き
        if na < a {
            // UFO と遮蔽物の天辺の傾きがより緩やか= ビルのより高い場所に登らないといけない
            a = na
        }
    }

    // y=ax+b, b=y-ax, where x=d
    println!("{:.12}", h - a * d);
}
