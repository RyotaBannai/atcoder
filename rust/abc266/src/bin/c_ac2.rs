use alga::general::ComplexField;
use proconio::{fastout, input, marker::Chars};
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
 * C - Convex Quadrilateral
 *
 * https://atcoder.jp/contests/abc266/tasks/abc266_c
 *
 * 参考
 * https://atcoder.jp/contests/abc266/editorial/4723
 *
 */

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vector {
    x: f64,
    y: f64,
}
impl Vector {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/*
y=mx+dより
m=(y2-y1)/(x2-x1)
点と直線の距離より
d=|ax+by+c|/√..
点と直線の距離について、直線の別の領域に存在すると符号が変わる（多分）
y=mx+d を原点に並行移動すると考えると
対象の点は (x,y) は (x-x1), (y-y1), y=mx になる,
よって d=ax+by, d=(y-y1)+(y2-y1)/(x2-x1)*(x-x1), を整理して
(x2-x1)*(y-y1)+(y2-y1)*(x-x1) を比較する
*/
fn d(v1: Vector, v2: Vector, v: Vector) -> f64 {
    let (x, y) = (v.x, v.y);
    (v2.x - v1.x) * (y - v1.y) - (v2.y - v1.y) * (x - v1.x)
}

fn sgn(x: f64) -> f64 {
    if x == 0. {
        0.
    } else {
        x / x.abs()
    }
}

#[fastout]
fn main() {
    input! {
        v:[(f64, f64); 4]
    }

    let (v1, v2, v3, v4) = (
        Vector::new(v[0].0, v[0].1),
        Vector::new(v[1].0, v[1].1),
        Vector::new(v[2].0, v[2].1),
        Vector::new(v[3].0, v[3].1),
    );

    let dist1 = (sgn(d(v1, v3, v2)), sgn(d(v1, v3, v4)));
    let dist2 = (sgn(d(v2, v4, v1)), sgn(d(v2, v4, v3)));
    println!(
        "{}",
        if dist1.0 * dist1.1 < 0. && dist2.0 * dist2.1 < 0. {
            "Yes"
        } else {
            "No"
        }
    );
}
