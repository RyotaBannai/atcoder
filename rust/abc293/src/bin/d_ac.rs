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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<usize, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;

use library::structure::disjoint_set::*;

/**
 * Tying Rope
 *
 * https://atcoder.jp/contests/abc293/tasks/abc293_d
 *
 * tags: #union_find #dfu #disjoint_set
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [(usize, char, usize, char); m]
    }

    let mut count = Set::new();
    let mut ds = DisjointSet::new(n - 1);
    for (a, _, c, _) in abcd {
        if ds.same(a - 1, c - 1) {
            count.insert(ds.p[a - 1]);
        } else {
            ds.unite(a - 1, c - 1);
        }
    }

    println!("{} {}", count.len(), n - m);
}
