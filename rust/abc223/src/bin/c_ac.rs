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
 * C - Doukasen
 *
 * https://atcoder.jp/contests/abc223/tasks/abc223_c
 *
 * 1cm 進むのに要する時間を管理して、
 * 1cm ずつ距離と 進む時間を加算していく.
 *
 * 加算した時間が合計時間/2 になった時点で加算した距離cm を出力する.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [(f64, f64); n]
    }
    let mut total = 0.;
    let mut ts = vec![]; // 1cm 進むのに要する時間
    for (a, b) in xs {
        ts.append(&mut vec![1. / b; a as usize]);
        total += a / b;
    }

    // println!("{:?}", &total);
    // println!("{:?}", &ts);

    let mut sum = 0.;
    let mut cur = 0.;
    for t in ts {
        if cur + t < total / 2. {
            cur += t;
            sum += 1.;
        } else if cur + t == total / 2. {
            println!("{}", sum + t);
            return;
        } else {
            let c = (total / 2. - cur).abs();
            let d = (cur + t - total / 2.).abs();
            println!("{}", sum + 1. * (c / (c + d)));
            return;
        }
    }
}
