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
 * Divide the Problems
 *
 * https://atcoder.jp/contests/abc132/tasks/abc132_c
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut d: [usize;n]
    }
    d.sort_unstable();
    if n % 2 != 0 {
        println!("0");
    }

    println!("{}", d[n / 2] - d[n / 2 - 1]);
}
