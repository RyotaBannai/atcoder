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
 * D - Freefall
 *
 * https://atcoder.jp/contests/abc279/tasks/abc279_d
 *
 * tags: #三分探索 #凸関数 #凸 #convex
 *
 * 関数が凸だと、二分探索だと誤差が生じる.
 *
 */

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64
    }

    let f = |n: usize| n as f64 * b + (a / (1. + n as f64).sqrt());

    if a <= b {
        println!("{:.10}", f(0));
        return;
    }
    let mut r = (a / b) as usize; // これは気づこう
    let mut l = 0usize;
    while r - l > 3 {
        // l=0,r=3 の時、m1=1,m2=2
        let m1 = (2 * l + r) / 3;
        let m2 = (l + 2 * r) / 3;
        let nx1 = f(m1);
        let nx2 = f(m2);

        if nx1 < nx2 {
            r = m2;
        } else {
            l = m1;
        }
    }

    // 範囲を順分に小さくしたら、その範囲を全探索すれば良い.
    let mut ans = std::f64::MAX;
    for n in l..=r {
        ans = ans.min(f(n));
    }
    println!("{:.10}", ans);
}
