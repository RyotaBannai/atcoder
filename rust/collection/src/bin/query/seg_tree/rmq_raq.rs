/**
 * @cpg_dirspec rmq_raq
 *
 * cpg run -p src/bin/query/seg_tree/rmq_raq.rs
 */
// use proconio::{fastout, input, marker::Chars};
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
use library::{query::seg_tree::*, utils::read::*};

/**
 * Range Add Query (RAQ) and Range Minimum Query(RMQ)
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_H
 *
 * tags: #segment_tree #セグメント木 #セグ木 #セグメントツリー
 *
 */

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, q) = (a[0], a[1]);

    let mut seg = LazySegTree::new(
        n,
        0,
        std::isize::MAX, // 注意: query の無効値としての単位元は fx:min なら MAX, fx:max なら MIN
        0,               // 注意: add 用だから単位元は 0
        std::isize::MAX,
        |a: isize, b: isize| a.min(b),
        |a: isize, b: isize, n: usize| a + b * n as isize,
        |_: isize, b: isize, n: usize| b * n as isize,
        |a: isize, b: isize| a + b,
        |_: isize, b: isize| b,
        |a: isize, x: isize| a > x,
    );

    for _ in 0..q {
        let b = read::<isize>();
        let t = b[0];
        if t == 0 {
            let (l, r, x) = (b[1] as usize, b[2] as usize, b[3]);
            seg.add(l, r + 1, x);
        } else {
            let (l, r) = (b[1] as usize, b[2] as usize);
            println!("{}", seg.query(l, r + 1));
        }
    }
}
