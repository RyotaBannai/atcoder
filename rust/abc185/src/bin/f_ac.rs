use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
// max, min,
// Ordering::{Equal, Greater, Less},
// };
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::query::seg_tree::*;

/**
 * F - Range Xor Query
 *
 * https://atcoder.jp/contests/abc185/tasks/abc185_f
 *
 * tags: #seg_tree #xor
 */

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
        qs: [(usize,usize, usize); q],
    }

    let mut seg = LazySegTree::new(
        n,
        0,
        0,
        0,
        0,
        |a: isize, b: isize| a ^ b,                      // xor
        |_: isize, _: isize, _: usize| unimplemented!(), // 使わない
        |a: isize, b: isize, _: usize| a ^ b,
        |_: isize, _: isize| unimplemented!(), // 使わない
        |a: isize, b: isize| a ^ b,
        |a: isize, x: isize| a > x,
    );

    for i in 0..n {
        seg.set(i, a[i]);
    }
    seg.build();

    for (t, x, y) in qs {
        if t == 1 {
            seg.update(x - 1, x, y as isize); // 1<=x
        } else {
            println!("{}", seg.query(x - 1, y));
        }
    }
}
