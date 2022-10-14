use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use typical90::query_lib::*;

/**
 *  028 - Cluttered Paper（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_ab
 *
 * セグ木を使って更新処理コストを減らす？
 *
*/

#[fastout]
fn main() {
    input! {
        k: usize,
        s: [(usize,usize, usize,usize); k]
    }

    let n = 1005;

    let f = |a: isize, b: isize| a + b;
    let mut vs = vec![];
    for i in 0..n {
        vs.push(LazySegTree::new(
            n,
            0,
            0,
            0,
            f,
            f,
            f,
            |a: isize, n: usize| a * n as isize,
            |a: isize, x: isize| a > x,
        ));
    }

    for (x1, y1, x2, y2) in s {
        for i in y1 - 1..y2 - 1 {
            vs[i].update(x1 - 1, x2 - 1, 1);
        }
    }

    let mut sum_x = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n - 1 {
            sum_x[i][j + 1] = vs[i].query(0, j);
        }
    }

    // println!("{:?}", &sum_x);
    // println!("{:?}", &x.dat);
    // println!("{:?}", &y.dat);
    // 各 k 回重なる部分の総和を求める
    // k: k 回重なる, v: 総和
    let mut sum: BTreeMap<usize, usize> = BTreeMap::new();
    for xs in sum_x {
        for i in 1..n {
            let v = (xs[i] - xs[i - 1]) as usize;
            if let Some(x) = sum.get_mut(&v) {
                *x += 1;
            } else {
                sum.insert(v, 1);
            }
        }
    }
    // println!("{:?}", &sum);

    // i=1 回重なる総和から順位出力
    for i in 1..=k {
        if let Some(x) = sum.get(&i) {
            println!("{}", x);
        } else {
            println!("0");
        }
    }
}
