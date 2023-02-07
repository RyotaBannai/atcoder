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

use library::{chmax, query::sparse_table::*};

/**
 * Mandarin Orange
 *
 * tags: #sparse_table
 *
 */

fn main() {
    input! {
        n: usize,
        a: [isize; n]
    }

    let mut ma = -1isize;
    let st = SparseTable::new(a);
    for l in 0..n {
        for r in l..n {
            chmax!(ma, (r - l + 1) as isize * st.query(l, r));
        }
    }

    println!("{}", ma);
}
