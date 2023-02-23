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
 * Popular Vote
 *
 * https://atcoder.jp/contests/abc161/tasks/abc161_b
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n]
    }

    a.sort_unstable();
    let s = a.iter().cloned().sum();
    for x in a.into_iter().rev().take(m) {
        if 4 * m * x < s {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
