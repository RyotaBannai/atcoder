use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;s
// use std::collections::{BinaryHeap, VecDeque};

/**
 * ASCII code
 *
 * https://atcoder.jp/contests/abc252/tasks/abc252_a
*/

fn convert(c: char, n: usize) -> char {
    (b'a' + (c as u8 - b'a' + n as u8)) as char
}

// #[fastout]
fn main() {
    input! { n: usize}
    println!("{}", convert('a', n - 97));
}
