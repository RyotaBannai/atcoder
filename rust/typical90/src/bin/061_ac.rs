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
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * 061 - Deck（★2）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bi
 *
 */

#[fastout]
fn main() {
    input! {
        q: usize,
        t: [(usize, usize); q],
    }
    let mut d = VecDeque::new();
    for (t, x) in t {
        match t {
            1 => d.push_back(x),
            2 => d.push_front(x),
            3 => println!("{}", d[d.len() - x]),
            _ => unreachable!(),
        }
    }
}
