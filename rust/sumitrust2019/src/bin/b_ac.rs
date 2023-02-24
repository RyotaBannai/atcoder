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
 * Tax Rate
 *
 * https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_b
 *
 * O(N) で全部試す.
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize
    }
    for i in 1..=50000 {
        if (i as f64 * 1.08 + 1e-5) as usize == n {
            println!("{}", i);
            return;
        }
    }
    println!(":(");
}
