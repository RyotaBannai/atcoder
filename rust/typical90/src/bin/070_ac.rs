use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
 * 070 - Plant Planning（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_br
 *
 * tags: #中央値 #median #絶対値最小
 *
 * 中央値で絶対値最小となる
 * 二分探索では小数は扱えないため求まらない.
 *
 *
 * 類題：
 * - ABC102-C 「Linear Approximation」
 * - ARC122-B 「Insurance」
 * - ARC071-D 「井井井」
 * - ABC127-E 「Cell Distance」
 * - JOI2008 春合宿 「Abduction」
 * - Tenka1 Programming Contest 2017 E
 * - 「CARtesian Coodinate」
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        ps: [(isize, isize); n]
    }

    let mut xs = ps.iter().map(|p| p.0).collect::<Vec<isize>>();
    let mut ys = ps.iter().map(|p| p.1).collect::<Vec<isize>>();

    xs.sort_unstable();
    ys.sort_unstable();

    let len = ps.len();

    let med_x = if xs.len() % 2 == 0 {
        (xs[len / 2 - 1] + xs[len / 2]) as f64 / 2.
    } else {
        xs[len / 2] as f64
    };

    let med_y = if ys.len() % 2 == 0 {
        (ys[len / 2 - 1] + ys[len / 2]) as f64 / 2.
    } else {
        ys[len / 2] as f64
    };

    let mut sum = 0f64;
    for x in xs {
        sum += (x as f64 - med_x).abs();
    }

    for y in ys {
        sum += (y as f64 - med_y).abs();
    }

    println!("{}", sum);
}
