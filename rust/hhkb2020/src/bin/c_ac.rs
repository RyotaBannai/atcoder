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
use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Neq Min
 *
 * https://atcoder.jp/contests/hhkb2020/tasks/hhkb2020_c
 *
 * i 回目の操作でまだ現れてない最小の整数n を高速に取得できるデータ構造ならなんでも良い.
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut s = Set::from_iter(0..200_002);
    for x in a {
        s.remove(&x);
        if let Some(y) = s.iter().next() {
            println!("{}", y);
        }
    }
}
