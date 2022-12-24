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

use library::geometry::{vector::place::place, vector::prelude::*};

/**
 * C - Triangle?
 *
 * https://atcoder.jp/contests/abc224/tasks/abc224_c
 *
 * tags: #幾何学 #geometry #小数 #三角形 #triangel
 *
 * 全ての組み合わせを作って、
 * 3点が時計回り、反時計回りになっていれば面積ができる.
 *
 *
 * 大きい数字の小数の扱いには気を付ける.
 * 例えば、D - Circumferences https://atcoder.jp/contests/abc259/tasks/abc259_d
 *
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        ps: [(f64, f64); n]
    }

    let mut ans = 0;
    // 3点が同一直線上になければ面積を持つ
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let v = Vector::new(ps[i].0, ps[i].1);
                let v2 = Vector::new(ps[j].0, ps[j].1);
                let v3 = Vector::new(ps[k].0, ps[k].1);
                let p = place(v, v2, v3);
                if p == 1 || p == 3 {
                    // 同一直線上にない
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
