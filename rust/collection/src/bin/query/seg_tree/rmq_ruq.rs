/**
 * @cpg_dirspec rmq_ruq
 *
 * cpg run -p src/bin/query/seg_tree/rmq_ruq.rs
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
 * Range Update Query (RUQ) and Range Minimum Query (RMQ)
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_F
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
        (1 << 31) - 1,
        (1 << 31) - 1,
        (1 << 31) - 1,
        (1 << 31) - 1,
        |a: isize, b: isize| a.min(b),
        |a: isize, b: isize, n: usize| a + b * n as isize,
        |_: isize, b: isize, n: usize| b * n as isize,
        |a: isize, b: isize| a + b,
        |_: isize, b: isize| b,
        |a: isize, x: isize| a > x,
    );

    for _ in 0..q {
        let b = read::<usize>();
        let t = b[0];
        if t == 0 {
            let (l, r, x) = (b[1], b[2], b[3]);
            seg.update(l, r + 1, x as isize);
        } else {
            let (l, r) = (b[1], b[2]);
            println!("{}", seg.query(l, r + 1));
        }
    }
}
