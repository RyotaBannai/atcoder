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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use practice2::query_lib::*;

/**
 * Segment Tree
 *
 * https://atcoder.jp/contests/practice2/tasks/practice2_j
 *
 * tags: #segment_tree #セグメント木 #セグ木 #セグメントツリー
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
        qs: [(usize, (usize, usize)); q]

    }
    let mut sg = LazySegTree::new(
        n,
        -1,
        -1,
        |a: isize, b: isize| a.max(b), // min
        |_: isize, b: isize| b,        // replace
        |_: isize, b: isize| b,        // replace
        |a: isize, _: usize| a,        // mul 1
        |a: isize, x: isize| a < x,
    );
    for (i, x) in a.iter().enumerate() {
        sg.set(i, *x);
    }
    sg.build();
    // println!("{:?}", sg.dat);

    for (t, p) in qs {
        match t {
            1 => {
                let (x, v) = p;
                sg.update(x - 1, x, v as isize);
            }
            2 => {
                let (l, r) = p;
                println!("{}", sg.query(l - 1, r));
            }
            3 => {
                let (x, v) = p;
                let a = sg.find_leftest(x - 1, n, v as isize);
                println!("{}", if a == -1 { n as isize } else { a } + 1);
                // println!("{} {:?}", y, sg.dat);
            }
            _ => {}
        }
    }
}
