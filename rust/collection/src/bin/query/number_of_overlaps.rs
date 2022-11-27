/**
 * @cpg_dirspec number_of_overlaps
 *
 * cpg run -p src/bin/query/number_of_overlaps.rs
 */
// use proconio::{fastout, input, marker::Chars};
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
use library::utils::read::*;

/**
 * The Maximum Number of Overlaps
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/5/DSL_5_B
 *
 * tags: #いもす法 #imos
 *
 */

// #[fastout]
fn main() {
    let k = read::<usize>()[0];
    let mut s = vec![];
    for _ in 0..k {
        let b = read::<usize>();
        s.push(((b[0], b[1]), (b[2], b[3]))); // (左上, 右下)
    }

    let n = 1005;
    let mut v = vec![vec![0isize; n]; n];
    // 簡単のためinput そのまま使う. 1-index
    // 入力が左下、右上なので、普通の平面座標とy軸の向きを反対方向にして考える
    for ((x1, y1), (x2, y2)) in s {
        v[y1][x1] += 1; // 内側
        v[y2][x1] -= 1; // 外側 左下
        v[y2][x2] += 1; // 外側 右下
        v[y1][x2] -= 1; // 外側 右上
    }
    // for i in 0..v.len() {
    //     println!("{:?}", &v[i]);
    // }

    // 行方向に累積和
    for i in 0..n - 1 {
        for j in 0..n - 1 {
            v[i][j + 1] += v[i][j];
        }
    }
    // 列方向に累積和
    for i in 0..n - 1 {
        for j in 0..n - 1 {
            v[j + 1][i] += v[j][i];
        }
    }

    let mut ma = 0;
    for i in 0..n {
        for j in 0..n {
            ma = ma.max(v[i][j]);
        }
    }

    println!("{}", ma);
}
