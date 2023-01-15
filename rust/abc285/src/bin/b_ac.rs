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
 * B - Longest Uncommon Prefix
 *
 * https://atcoder.jp/contests/abc285/tasks/abc285_b
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }

    for k in 1..n {
        let mut count = 0;
        for i in 0..n {
            if i + k < n && s[i] != s[i + k] {
                count += 1;
                continue;
            }
            break;
        }
        println!("{}", count);
    }
}
