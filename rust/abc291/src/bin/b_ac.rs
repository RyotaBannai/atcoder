use itertools::Itertools;
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
 * Trimmed Mean
 *
 * https://atcoder.jp/contests/abc291/tasks/abc291_b
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n*5]
    }
    println!(
        "{:.10}",
        a.into_iter()
            .sorted()
            .skip(n)
            .take(3 * n)
            .map(|x| x as f64)
            .sum::<f64>()
            / (3. * n as f64)
    );
}
