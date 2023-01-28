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
 * Majority
 *
 * https://atcoder.jp/contests/abc287/tasks/abc287_a
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut count = 0;
    for xs in s {
        if xs[0] == 'F' {
            count += 1;
        }
    }
    if count > n / 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
