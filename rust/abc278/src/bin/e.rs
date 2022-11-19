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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * E - Grid Filling
 *
 * https://atcoder.jp/contests/abc278/tasks/abc278_e
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        hh: usize,
        ww: usize,
        a: [[usize ;w]; h],
    }

    // 行ごとに絵処理 window 以外にある整数の種類を管理
    let mut v = vec![vec![0; w - ww]; h - hh];
    for i in 0..v.len() {
        // i 行目を処理
        for k in 0..w - ww {
            let mut set = Set::new();
            for j in 0..k {
                set.insert(a[i][j]);
            }
            for j in k + ww..w {
                set.insert(a[i][j]);
            }
            v[i][k] = set.len();
        }
    }

    // 行ごとに整数の種類を管理
    let mut sum = vec![0; h];
    for i in 0..v.len() {
        let mut set = Set::new();
        for j in 0..w {
            set.insert(a[i][j]);
        }
        sum[i] = set.len();
    }

    // 行ごとの被り発生

    for k in 0..w - ww {
        let mut set = Set::new();
        // for i in
        // v[i][k] = set.len();
    }
}
