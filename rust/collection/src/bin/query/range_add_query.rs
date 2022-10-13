/**
 * @cpg_dirspec range_add_query
 *
 * cpg run -p src/bin/query/range_add_query.rs
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
use collection::{query_lib::*, utils::*};

/**
 * Range Add Query (RAQ)
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_E
 *
 * tags: #segment_tree #セグメント木 #セグ木 #セグメントツリー
 *
 */

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, q) = (a[0], a[1]);

    let f = |a: isize, b: isize| a + b;
    let mut seg = LazySegTree::new(
        n,
        0,
        0,
        0,
        f,
        f,
        f,
        |a: isize, n: usize| a * n as isize,
        |a: isize, x: isize| a > x,
    );

    for _ in 0..q {
        let b = read::<usize>();
        let t = b[0];
        if t == 0 {
            let (l, r, x) = (b[1], b[2], b[3]);
            seg.update(l - 1, r, x as isize);
        } else {
            let i = b[1];
            println!("{}", seg.query(i - 1, i));
        }
    }
}
