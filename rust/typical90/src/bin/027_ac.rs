use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 027 - Sign Up Requests （★2）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_aa
 *
 * tags: #辞書 #map
 *
*/

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut m = Map::new();
    for (i, x) in s.iter().enumerate() {
        if m.get(x).is_none() {
            m.insert(x.to_string(), i);
        }
    }
    let mut v = m.iter().collect_vec();
    v.sort_unstable_by(|a, b| a.1.cmp(b.1));
    for x in v {
        println!("{}", x.1 + 1);
    }
}
