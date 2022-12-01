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

use library::{chmax, chmin};

/**
 * B - Light It Up
 *
 * https://atcoder.jp/contests/abc255/tasks/abc255_b
 *
 * N が大きくないから、全組み合わせを試す.
 *
 * ある人について、かく明かりから一番近いものを探してそれだけを管理する.
 * かく人について探し終えたら、その中から最大となる距離を見つける. それが、
 * すべての人が少なくとも 1 つの明かりによって照らされるために必要な明かりの強さとなる
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
        xy: [(f64, f64); n],
    }

    let max = 10000000.;
    let mut mi = vec![max; n];
    // 各点と、明かりの座標の全組み合わせを試せば良い.
    for (i, &(x, y)) in xy.iter().enumerate() {
        for &j in a.iter() {
            let (nx, ny) = xy[j - 1];
            let r = ((x - nx) * (x - nx) + (y - ny) * (y - ny)).sqrt(); // 三平方の定理
            chmin!(mi[i], r);
        }
    }

    let mut ma = 0.;
    for &d in &mi {
        if d == max {
            panic!("should be smaller than ma");
        }
        chmax!(ma, d);
    }

    println!("{}", ma);
}
