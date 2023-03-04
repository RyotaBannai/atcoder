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
 * Ordinary Number
 *
 * https://atcoder.jp/contests/abc132/tasks/abc132_b
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize;n]
    }
    let mut count = 0;
    for i in 1..n - 1 {
        if a[i - 1] < a[i] && a[i + 1] < a[i] {
            // どっちも小さい
            continue;
        }
        if a[i - 1] < a[i] || a[i + 1] < a[i] {
            // どっちかのみ i より小さい
            count += 1;
        }
    }
    println!("{}", count);
}
