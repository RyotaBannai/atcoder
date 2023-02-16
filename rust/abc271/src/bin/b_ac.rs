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
 * Maintain Multiple Sequences
 *
 * https://atcoder.jp/contests/abc271/tasks/abc271_b
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize, q: usize,
    }

    let mut l = vec![vec![]; n];
    for i in 0..n {
        input! {
            m: usize,
            ll: [usize; m]
        }
        l[i] = ll;
    }
    input! {
        st: [(usize, usize);q]
    }
    for (s, t) in st {
        println!("{}", l[s - 1][t - 1]);
    }
}
