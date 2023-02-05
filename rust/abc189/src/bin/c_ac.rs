use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::{chmax, chmin};
use std::usize::MAX;

/**
 * Mandarin Orange
 *
 * tags: #全探索 #10^8
 *
 *
 */
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut ans = 0;
    for l in 0..n {
        let mut mi = MAX;
        for r in l..n {
            chmin!(mi, a[r]);
            chmax!(ans, mi * (r - l + 1));
        }
    }
    println!("{}", ans);
}
