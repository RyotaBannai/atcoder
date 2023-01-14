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
// type Set = BTreeSet<isize>;
// use easy_ext::ext;
use library::structure::rev_priority_queue::*;
use std::collections::{BinaryHeap, VecDeque};

/**
 * Querying Multiset
 *
 * https://atcoder.jp/contests/abc212/tasks/abc212_d
 *
 * tags: #priority_queue #heap
 *
 * 一気に加算する操作を別の変数で管理して、
 * 追加する時にはそれまでの総和から引いて追加、
 * 取り出すときは、それまでの総和を追加するとよい.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut hist = vec![];
    let mut added = 0;
    let mut s = BinaryHeap::new();
    for _ in 0..n {
        input! { m: usize }
        if m == 1 {
            input! { x: isize }
            s.push_rev(x - added);
        } else if m == 2 {
            input! { x: isize }
            added += x;
        } else if let Some(x) = s.pop_rev() {
            hist.push(x + added);
        }
    }
    for x in hist {
        println!("{}", x);
    }
}
