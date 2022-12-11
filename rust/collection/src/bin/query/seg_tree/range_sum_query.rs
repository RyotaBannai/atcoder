/**
 * @cpg_dirspec range_sum_query
 *
 * cpg run -p src/bin/query/seg_tree/range_sum_query.rs
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
use library::{query::*, utils::read::*};

/**
 * Range Sum Query (RSQ)
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_B
 *
 * tags: #segment_tree #セグメント木 #セグ木 #セグメントツリー
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_A と配列の index が違うため注意.
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
    let f = |a: isize, b: isize| a + b;
    let fp = |a: isize, n: usize| a * n as isize;
    let mut seg = LazySegTree::new(n, 0, 0, 0, f, f, f, fp, |a: isize, x: isize| a > x);

    for (t, q) in qs {
        if t == 0 {
            let (x, v) = q;
            seg.update(x - 1, x, v as isize);
        } else {
            let (l, r) = q;
            println!("{}", seg.query(l - 1, r));
        }
    }
}
