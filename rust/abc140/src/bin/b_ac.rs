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
 * Buffet
 *
 * https://atcoder.jp/contests/abc140/tasks/abc140_b
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        b: [usize;n],
        c: [usize;n-1],
    }
    let mut prev = 0;
    let mut sum = 0;
    for (i, x) in a.into_iter().enumerate() {
        sum += b[x - 1];
        if i != 0 {
            if prev + 1 == x {
                sum += c[prev - 1];
            }
        }
        prev = x;
    }
    println!("{}", sum);
}
