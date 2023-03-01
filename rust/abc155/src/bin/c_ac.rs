use itertools::Itertools;
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
type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::chmax;

/**
 * Poll
 *
 * https://atcoder.jp/contests/abc155/tasks/abc155_c
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut m = Map::new();
    for x in s {
        *m.entry(x).or_insert(0) += 1;
    }
    let mut ma = 0;
    for (k, &v) in m.iter() {
        chmax!(ma, v);
    }

    let mut ans = vec![];
    for (k, &v) in m.iter() {
        if v == ma {
            ans.push(k);
        }
    }
    for x in ans.into_iter().sorted() {
        println!("{}", x);
    }
}
