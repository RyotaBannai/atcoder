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
use std::{
    collections::{BinaryHeap, VecDeque},
    iter::FromIterator,
};

/**
 * E - Sorting Queries
 *
 * https://atcoder.jp/contests/abc217/tasks/abc217_e
 *
 * tags: #ソート #sort #ヒープ #heap #キュー #queue
 *
 * キューをヒープ２つ使う.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut a = VecDeque::new();
    let mut sorted: BinaryHeap<isize> = BinaryHeap::new();
    for _ in 0..n {
        input! { m: usize }
        if m == 1 {
            input! { x:isize }
            a.push_back(-x);
        } else if m == 2 {
            if let Some(x) = sorted.pop() {
                println!("{}", x.abs());
            } else if let Some(x) = a.pop_front() {
                println!("{}", x.abs());
            }
        } else {
            sorted.append(&mut BinaryHeap::from_iter(a.iter().cloned()));
            a.clear();
        }
    }
}
