use proconio::{fastout, input, marker::Chars};
use std::usize::MAX;
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet, HashSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use itertools::*;
use library::{
    geometry::{
        circle::{intersect::*, prelude::*},
        vector::{distance::*, prelude::*},
    },
    structure::disjoint_set::*,
};
use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Circumferences
 *
 * https://atcoder.jp/contests/abc259/tasks/abc259_d
 *
 * tags: #union_find #disjoint_set #circle #円 #幾何学 #geometry #小数
 *     
 * 1.
 * 始点と終点と交わる円を探す. この時点でなければ No, 同じ円なら Yes
 *
 * 全ての円に対して、交差している円どうしの隣接リストを作成する -> これはやらない. 隣接リストが 3000^3000 になる可能性がある.
 * (例えば、全てくっついている時など. dfs で使ったマスを埋めても、取り出す作業に計算がかかる.)
 * ★隣接リストを作って良いのは、隣接するマスが限定的な場合のみ.
 *
 * 地点A から地点B まで辿り着けるか？という順序やコスト最小を考えない場合は、UnionFind 一択.
 *
 * 2.
 * 大きい数値に対して float や double を使わない.
 * 10桁くらい以上の整数部分が容赦なく丸められる.
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        s: (f64, f64),
        t: (f64, f64),
        c: [((f64, f64), f64); n]
    }

    let cs = c
        .iter()
        .map(|((x, y), r)| Circle::new(Vector::new(*x, *y), *r))
        .collect::<Vec<_>>();

    // let v = Vector::new(s.0, s.1);
    // let w = Vector::new(t.0, t.1);
    let mut c1 = MAX;
    let mut c2 = MAX;

    for (i, &a) in cs.iter().enumerate() {
        let (v, r) = (a.c, a.r as isize);
        let (x, y) = (v.x as isize, v.y as isize);
        let (a, b) = (s.0 as isize, s.1 as isize);
        let (c, d) = (t.0 as isize, t.1 as isize);

        if (x - a) * (x - a) + (y - b) * (y - b) - r * r == 0 {
            c1 = i;
        }
        if (x - c) * (x - c) + (y - d) * (y - d) - r * r == 0 {
            c2 = i;
        }
    }

    if c1 == MAX || c2 == MAX {
        println!("No");
        return;
    }
    if c1 == c2 {
        println!("Yes");
        return;
    }

    let mut ds = DisjointSet::new(n);
    for (i, j) in iproduct!(0..n, 0..n) {
        if i == j {
            continue;
        }
        // is_intersect_circles(c1, c2)
        // ここらへんの 10^9 を２乗する計算（10^18 の小数部分）は、小数が丸められる
        let (v1, r) = (cs[i].c, cs[i].r as isize);
        let (a, b) = (v1.x as isize, v1.y as isize);
        let (v2, s) = (cs[j].c, cs[j].r as isize);
        let (c, d) = (v2.x as isize, v2.y as isize);
        let d2 = (a - c) * (a - c) + (b - d) * (b - d);
        let c2 = (r + s) * (r + s);
        if d2 + r < s || d2 + s < r || d2 > c2 {
            continue;
        }

        if !ds.same(i, j) {
            ds.unite(i, j);
        }
    }

    if ds.same(c1, c2) {
        println!("Yes");
    } else {
        println!("No");
    }

    // -1000000000 -831597471 s
    // -999974046 -494792414 t
    // -1000000000 -1000000000 168402529 c1
    // -999974046 -663194943 168402529 c2

    // let a = -1000000000isize;
    // let b = -831597471isize;
    // let c = 168402529isize;
    // let d = -999974046isize;
    // let e = -494792414isize;
    // let f = -663194943isize;

    // println!("{}", (a - a) * (a - a) + (a - b) * (a - b) - c * c); // s と c1 の距離 == 0
    // println!("{}", (d - d) * (d - d) + (f - e) * (f - e) - c * c); // t と c2 の距離 == 0
    // println!(
    //     "{} {}",
    //     (a - d) * (a - d) + (a - f) * (a - f), // 中心間の距離 - 2*円の半径
    //     (2 * c) * (2 * c)
    // ); // c1 と c2 の距離
}
