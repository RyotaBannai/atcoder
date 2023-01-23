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

use library::geometry::vector::{polar::*, prelude::*};

/**
 * Opposite
 *
 * https://atcoder.jp/contests/abc197/tasks/abc197_d
 *
 * tags: #geometry #幾何学 #ベクトル #回転 #polar
 *
 * 中心を求めてから、中心から a に向かうベクトルを反時計回りに　2pi/n だけ回転させたベクトルを
 * 中心のベクトルに加える（平行移動させる）と良い.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: (f64, f64),
        b: (f64, f64),
    }

    let st = Vector::new(a.0, a.1);
    let en = Vector::new(b.0, b.1);
    let c = (en + st) / 2.;

    let deg = 2. * std::f64::consts::PI / n as f64;
    let ans = polar_on_v(st - c, deg) + c;
    println!("{:.12} {:.12}", ans.x, ans.y);
}
