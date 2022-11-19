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

use abc278::query_lib::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
        q: usize
    }

    // RUQ
    let mut seg = LazySegTree::new(
        n,
        std::isize::MAX,
        std::isize::MAX,
        std::isize::MAX,
        |a: isize, b: isize| a.min(b),
        |_: isize, b: isize| b,
        |_: isize, b: isize| b,
        |a: isize, n: usize| a * n as isize,
        |a: isize, x: isize| a > x,
    );

    for (i, &x) in a.iter().enumerate() {
        seg.set(i, x);
    }
    seg.build();

    // println!("{:?}", seg.dat);

    for _ in 0..q {
        input! {
            t: usize
        }
        if t == 1 {
            // range update
            input! { x: isize }
            seg.update(0, n, x);
        } else if t == 2 {
            input! { i: usize, x: isize }
            let n = seg.query(i - 1, i);
            seg.update(i - 1, i, n + x);
        } else {
            input! { i: usize }
            println!("{}", seg.query(i - 1, i));
            // println!("{:?}", seg.dat);
        }
    }
}
