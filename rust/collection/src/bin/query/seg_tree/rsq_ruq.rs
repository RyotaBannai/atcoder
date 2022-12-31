/**
 * @cpg_dirspec rsq_ruq
 *
 * cpg run -p src/bin/query/seg_tree/rsq_ruq.rs
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
 * Range Update Query (RUQ) and Range Sum Query (RSQ)
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_I
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
        0,
        // 注意: es, em が異なるのは、0 で update をかける時 0 以外ではなく MAX (or Min)の時にしたいため.（判定時にem を参照）
        // add (RAQ)の場合は 0 以外としても問題ない.
        std::isize::MAX,
        0,
        |a: isize, b: isize| a.saturating_add(b),
        |a: isize, b: isize, n: usize| a + b * n as isize, // 注意: update
        |_: isize, b: isize, n: usize| b * n as isize,     // 注意: update, 前回の lazy を捨てる
        |a: isize, b: isize| a + b,
        |_: isize, b: isize| b,
        |a: isize, x: isize| a > x,
    );

    for _ in 0..q {
        let b = read::<isize>();
        let t = b[0];
        if t == 0 {
            let (l, r, x) = (b[1] as usize, b[2] as usize, b[3]);
            seg.update(l, r + 1, x);
        } else {
            let (l, r) = (b[1] as usize, b[2] as usize);
            println!("{}", seg.query(l, r + 1));
        }
    }
}
