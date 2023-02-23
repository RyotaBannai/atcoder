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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<String>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
/**
 * gacha
 *
 * https://atcoder.jp/contests/abc164/tasks/abc164_c
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut set = Set::new();
    for x in s {
        set.insert(x);
    }
    println!("{}", set.len());
}
