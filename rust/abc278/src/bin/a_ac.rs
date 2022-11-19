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
 * A - Shift
 *
 * https://atcoder.jp/contests/abc278/tasks/abc278_a
 */

#[fastout]
fn main() {
    input! {
        n : usize,
        k: usize,
        a: [usize; n]
    }

    for x in a.iter().skip(k) {
        print!("{} ", x);
    }
    for _ in 0..k.min(n) {
        print!("0 ");
    }

    println!();
}
