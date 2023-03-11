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
 * Call the ID Number
 *
 * https://atcoder.jp/contests/abc293/tasks/abc293_b
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [usize;n]
    }
    let mut s = Set::from_iter(1..=n);
    for i in 1..=n {
        if !s.contains(&i) {
            // 呼ばれていないなら何もしない
            continue;
        }

        s.remove(&xs[i - 1]);
    }
    println!("{}", s.len());
    for x in s {
        print!("{} ", x);
    }
}
