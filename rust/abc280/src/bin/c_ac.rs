use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use regex::internal::Char;
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

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    for i in 0..s.len() {
        if s[i] != t[i] {
            println!("{}", i + 1);
            return;
        }
    }

    println!("{}", s.len() + 1);
}
