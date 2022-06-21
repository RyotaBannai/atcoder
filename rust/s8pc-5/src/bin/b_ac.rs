use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max,
//     min,
//     Ordering::{Equal, Greater, Less},
// };
use flib::{fmax, fmin};
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
 * B - Emblem
 *
 * https://atcoder.jp/contests/s8pc-5/tasks/s8pc_5_b
 *
 * tags: #二分探索 #最小値の最大化
 *
 *
 * 円 n~n+m 個の円の半径は決まっていない
 * 半径固定の円も含めた円の最も小さい半径を最大化
 *
 * 円は内接して良いが、円どうしが交わったり、含んではならない。この判定は　半径 r の距離で判定
 * 　1. 円の接する or 交差しない条件： r1+r2 =< c1 c2 の距離
 * 　2. 円の交差条件：r1+r2 > c1 c2 の距離
 * 　3. 内接条件： r2 == r1 + c1 c2 の距離   r2 が大きい円の半径
 * 　4. 円の包含条件： r2 > r1 + c1 c2 の距離
 *
 * 上記４パターンのうち 2,3,4 はダメ (内接もダメ)
 *
 * n m
 * x1 y1 r1
 * x2 y2 r2
 * .
 * .
 * xN yN
 * x(N+1) y(N+1)
 * .
 * .
 * x(N+M) y(N+M)
 *
 *
 * 0 2
 * 6 3
 * 2 4
 * この場合、接するから、中心点間の距離/2 が最小半径の最大になる
 * 基本的には、接するだけ広げて最小半径を見る
 *
 * 固定なら　中心点間距離-固定円半径
 * 未定なら　中心点間距離/2
 *
 * r 以上の半径まで全て広げられるか？という問題に置き換えて二分探索で解く
 */

mod flib {
    pub fn fmin(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else {
            b
        }
    }

    pub fn fmax(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else {
            b
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        consts: [(isize, isize, f64); n],
        vals: [(isize, isize); m],
    }

    // 中心間距離の２乗
    let dist = |p1: (isize, isize), p2: (isize, isize)| -> f64 {
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f64
    };

    let check = |r: f64| -> (bool, f64) {
        let mut mi = std::f64::MAX;
        let mut ok = true;

        // 半径未定どうしの円の比較
        for i in 0..m {
            for j in i + 1..m {
                let d = dist(vals[i], vals[j]).sqrt() / 2f64;
                // dbg!(d);
                if d < r {
                    ok = false;
                } else {
                    mi = fmin(mi, d);
                }
            }
        }

        // 半径未定に対して、固定半径の円どうしの比較
        for i in 0..m {
            for j in 0..n {
                let (x2, y2, r2) = consts[j];
                let d = (dist(vals[i], (x2, y2)).sqrt() - r2).abs();
                if d < r {
                    ok = false;
                } else {
                    mi = fmin(mi, d);
                }
            }
        }

        // 固定半径の円どうしの比較
        for i in 0..n {
            for j in i + 1..n {
                let (x1, y1, r1) = consts[i];
                let (x2, y2, r2) = consts[j];
                let d = dist((x1, y1), (x2, y2)).sqrt();
                if d < r1 + r2 || r1 < r || r2 < r {
                    ok = false;
                } else {
                    mi = fmin(mi, fmin(r1, r2));
                }
            }
        }

        (ok, mi)
    };

    let mut ma = 0f64; // 0 から初めていて 0 に収束できないため、半径の最小が .5 の場合最適化できない. 初回に check を入れるか left を -1 にする
    let mut left = -1isize;
    let mut right = std::isize::MAX - 1;
    while right - left > 1 {
        let mid = (right + left) / 2;
        let (ok, mi) = check(mid as f64);
        if ok {
            left = mid;
            ma = fmax(mi, ma);
        } else {
            right = mid;
        }
    }

    println!("{}", ma);
}
