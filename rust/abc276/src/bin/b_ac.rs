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
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * B - Adjacency List
 *
 * https://atcoder.jp/contests/abc276/tasks/abc276_b
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut v = vec![Set::new(); n];
    for (a, b) in ab {
        v[a - 1].insert(b);
        v[b - 1].insert(a);
    }

    for s in v {
        print!("{} ", s.len());
        for x in s {
            print!("{} ", x);
        }
        println!();
    }
}
