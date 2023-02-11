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
type Map = BTreeMap<isize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use ordered_float::OrderedFloat;

/**
 *
 * Collinearity
 *
 * https://atcoder.jp/contests/abc181/tasks/abc181_c
 *
 * tags: #float #ordered_float
 *
 * 同一直線上は、x,y 軸に並行なもの以外に、傾きと切片が同じ点同士も同一直線上にあるとみなすことができる
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n]
    }

    // x or y 座標が３つ以上同じ点があるかどうかチェック
    let mut mx = Map::new();
    let mut my = Map::new();
    for &(x, y) in &xy {
        *mx.entry(x).or_insert(0) += 1;
        *my.entry(y).or_insert(0) += 1;
    }
    for m in &[mx, my] {
        for (_, &v) in m.iter() {
            if v >= 3 {
                println!("Yes");
                return;
            }
        }
    }

    // ある３点間の傾きが同じであるかどうかチェック
    let mut m: BTreeMap<(OrderedFloat<f64>, OrderedFloat<f64>), usize> = BTreeMap::new();
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];
            let xp = x2 - x1;
            let yp = y2 - y1;
            // すでにチェック済み
            if xp == 0 || yp == 0 {
                continue;
            }
            // xの係数と切片で同一のものを管理
            // yの係数で割る
            let k = (
                OrderedFloat(yp as f64 / xp as f64),
                OrderedFloat((y1 * xp - x1 * yp) as f64 / yp as f64), // y=(y2-y1/x2-x1)x+b // y 側の倍数に依存するから、y でわる
            );
            *m.entry(k).or_insert(0) += 1;
        }
    }
    for (_, &v) in m.iter() {
        if v >= 3 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
