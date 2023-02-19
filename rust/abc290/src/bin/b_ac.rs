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
 * Qual B
 *
 * https://atcoder.jp/contests/abc290/tasks/abc290_b
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }
    let mut count = 0;
    for c in s {
        if c == 'o' && count + 1 <= k {
            count += 1;
            print!("o");
        } else {
            print!("x");
        }
    }
    println!();
}
