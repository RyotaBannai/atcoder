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
 * Tap Dance
 *
 * https://atcoder.jp/contests/abc141/tasks/abc141_b
 *
 */

// #[fastout]
fn main() {
    input! {
        s: Chars
    }
    for i in 1..=s.len() {
        if i % 2 == 0 && s[i - 1] == 'R' {
            println!("No");
            return;
        }
        if i % 2 != 0 && s[i - 1] == 'L' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
