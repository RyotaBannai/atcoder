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
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [(f64, f64); n]
    }
    let mut total = 0.;
    for (a, b) in &xs {
        total += a / b;
    }

    let mut sum = 0.;
    let mut t = total / 2.; // 左右どちらから始めた場合も、合計時間の半分使える
    for (a, b) in &xs {
        sum += a.min(t * b); // 時間t がまだ十分に余ってるなら、a cm進めるが、t が不足してる場合は、t*b 分だけしか進めない.
        t -= (a / b).min(t); // 0. 以下にならないように調整する. a cm を b cm/s 進む時に費やす時間を引く. もし不足してる時は、t しか使わない（この時　残りがちょうど 0 s となる）
    }
    println!("{}", sum);
}
