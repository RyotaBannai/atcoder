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
 * Inverse of Permutation
 *
 * https://atcoder.jp/contests/abc217/tasks/abc217_c
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        ps: [usize;n]
    }
    let mut v = vec![-1; n];
    for (i, &p) in ps.iter().enumerate() {
        v[p - 1] = (i + 1) as isize;
    }
    for x in v.iter() {
        print!("{} ", x);
    }
    println!();
}
