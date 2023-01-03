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
use std::f64::consts::PI;

/**
 * 高橋君ボール1号 (水色)
 *
 * https://atcoder.jp/contests/abc026/tasks/abc026_d
 *
 * tags: #二分探索
 *
 * 初期値 α,β を求める
 * ある区間α,β で
 * f(α)<100
 * f(β)>100
 *
 * のように f(t)=100 を挟み込む区間を決める
 * （中間値の定理により、f(t)=100 となる位置を必ず含む区間ならどこでも良い）
 *
 * t=200 などとすれば
 * f(200)=200A+Bsin(Cπt)≥200−100≥100　(問題の制約条件の A≥1, B≤100 より)
 * となるから、r=200 以上にすればよい.
 *
 * 参考
 * https://perogram.hateblo.jp/entry/abc026_d
 * https://drken1215.hatenablog.com/entry/2021/07/03/173400
 */
// #[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64
    }

    let calc = |t: f64| a * t + b * (c * t * PI).sin();

    let mut l = 0.;
    let mut r = 200.;
    for _ in 0..100 {
        let t = (r + l) / 2.;
        let res = calc(t);
        if res < 100. {
            l = t;
        } else {
            r = t;
        }
    }

    println!("{}", r);
    // println!("{} {}", l, r);
}
