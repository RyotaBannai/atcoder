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
 * Power Socket
 *
 * https://atcoder.jp/contests/abc139/tasks/abc139_b
 *
 * b==1 の時は不要
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        a: usize,
        b: usize
    }

    let mut count = 0;
    let mut sum = 1;
    while sum < b {
        sum -= 1;
        sum += a;
        count += 1;
    }
    println!("{}", count);
}
