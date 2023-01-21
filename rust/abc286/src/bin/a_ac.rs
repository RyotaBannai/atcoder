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

/**
 * Range Swap
 *
 * https://atcoder.jp/contests/abc286/tasks/abc286_a
 *
 */

// #[fastout]
fn main() {
    input! {
        mut n: usize,
        mut p: usize,
        mut q: usize,
        mut r: usize,
        mut s: usize,
        list: [usize; n]
    }

    p -= 1;
    q -= 1;
    r -= 1;
    s -= 1;
    for i in 0..p {
        print!("{} ", list[i]);
    }
    for x in list[r..=s].iter() {
        print!("{} ", x);
    }
    for i in q + 1..r {
        print!("{} ", list[i]);
    }

    for x in list[p..=q].iter() {
        print!("{} ", x);
    }
    for i in s + 1..n {
        print!("{} ", list[i]);
    }
    println!();
}
