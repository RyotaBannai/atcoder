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
 * ROT N
 *
 * https://atcoder.jp/contests/abc146/tasks/abc146_b
 *
 */
// #[fastout]
fn main() {
    input! {
        n: u8,
        s: Chars
    }
    for c in s {
        print!("{}", ((c as u8 + n - b'A') % 26 + b'A') as char);
    }
}
