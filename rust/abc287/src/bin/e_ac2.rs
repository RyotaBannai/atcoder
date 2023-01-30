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
 *
 * Karuta
 *
 * https://atcoder.jp/contests/abc287/tasks/abc287_e
 *
 *
 * tags: #trie #トライ木
 *
 *
 */
use library::graph::trie::*;

// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [Chars; n]
    }
    let mut tri = Trie::new('a');
    for (i, x) in xs.iter().enumerate() {
        tri.insert(x, i)
    }
    for x in xs.iter() {
        println!("{}", tri.find_gcss(x).len());
    }
}
