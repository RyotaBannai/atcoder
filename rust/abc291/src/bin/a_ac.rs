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
 * camel Case
 *
 * https://atcoder.jp/contests/abc291/tasks/abc291_a
 *
 */
// #[fastout]
fn main() {
    input! {
        s:Chars
    }
    for (i, c) in s.into_iter().enumerate() {
        if ('A'..='Z').contains(&c) {
            println!("{}", i + 1);
            return;
        }
    }
}
