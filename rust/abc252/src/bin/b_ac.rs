use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
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
 * Takahashi's Failure
 *
 * https://atcoder.jp/contests/abc252/tasks/abc252_b
*/

#[fastout]
fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize; n],
        b:[usize; k],
    }

    let mut ma = 0;
    for &x in &a {
        ma = max(ma, x)
    }

    let mut v = vec![];
    for (i, x) in a.iter().enumerate() {
        if *x == ma {
            v.push(i + 1);
        }
    }

    for x in &v {
        if b.contains(x) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
