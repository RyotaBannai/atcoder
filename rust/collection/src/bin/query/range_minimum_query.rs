/**
 * @cpg_dirspec range_minimum_query
 *
 * cpg run -p src/bin/query/range_minimum_query.rs
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
use collection::{query::*, utils::read::*};

/**
 * Range Minimum Query (RMQ)
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_A
 *
 * tags: #segment_tree #セグメント木 #セグ木 #セグメントツリー
 *
 */

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, q) = (a[0], a[1]);
    let mut qs = vec![];
    for _ in 0..q {
        let b = read::<usize>();
        qs.push((b[0], (b[1], b[2])));
    }

    let mut seg = LazySegTree::new(
        n,
        (1 << 31) - 1,
        (1 << 31) - 1,
        (1 << 31) - 1,
        |a: isize, b: isize| a.min(b), // min
        |_: isize, b: isize| b,        // replace
        |_: isize, b: isize| b,        // replace
        |a: isize, _: usize| a,        // mul 1
        |a: isize, x: isize| a > x,
    );

    for (t, q) in qs {
        if t == 0 {
            let (x, v) = q;
            seg.update(x, x + 1, v as isize);
        } else {
            let (l, r) = q;
            println!("{}", seg.query(l, r + 1));
        }
    }
}
