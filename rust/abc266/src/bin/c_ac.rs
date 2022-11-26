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
 * tags: #幾何学 #geometry #外積
 *
 * もし凸多角形（全ての内角が 180 度以下）なら、線分同士が全て交差する条件を使うのがシンプル
 *
 * 他の解法として、全ての頂点に対して、ひとつ前のベクトルを始線とした時に、現在のベクトルの位置が外積で正であれば（0<θ<π）良いとする方法もある
 * （正方形abcd において、始線 cb に対し、ab の外積が正、これを各頂点について調べる）
 * https://atcoder.jp/contests/abc266/editorial/4659
 */
use abc266::geometry::*;

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

    println!(
        "{}",
        if VectorFns::intersect(v1, v3, v2, v4) {
            "Yes"
        } else {
            "No"
        }
    );
}
