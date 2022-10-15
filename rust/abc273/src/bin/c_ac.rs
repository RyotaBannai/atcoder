use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * https://atcoder.jp/contests/abc273/tasks/abc273_c
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        xs: [usize;n]
    }
    let mut s = Set::new();
    for &x in &xs {
        s.insert(x);
    }
    let vs = s.into_iter().collect::<Vec<_>>();

    let mut v = vec![0; n]; // k は 0..n-1
    let k = vs.len();
    for &x in &xs {
        let pos = vs.lower_bound(&x);
        v[k - pos - 1] += 1;
    }

    for k in 0..n {
        println!("{}", v[k])
    }
}
