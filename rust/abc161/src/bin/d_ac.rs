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
type Map = BTreeMap<String, bool>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use library::{max, min};
use std::collections::{BinaryHeap, VecDeque};

/**
 * Lunlun Number
 *
 * https://atcoder.jp/contests/abc161/tasks/abc161_d
 *
 * tags: #bfs
 *
 * 高々 k<=10^5 の取り出しだから愚直に実装して間に合う.
 *
 */

// #[fastout]
fn main() {
    input! {
        k: usize
    }

    let mut ma = Map::new();
    let mut q = VecDeque::new(); // 現在の数値と１の桁の数値
    for i in 1..10 {
        q.push_back((format!("{}", i), i));
    }
    let mut count = 0;
    while let Some((s, i)) = q.pop_front() {
        count += 1;
        if count == k {
            println!("{}", s);
            return;
        }

        for &j in &[max!(0, i - 1), i, min!(9, i + 1)] {
            let s = format!("{}{}", s, j);
            if ma.get(&s).is_none() {
                ma.entry(s.clone()).or_insert(true);
                q.push_back((s, j));
            }
        }
    }
}
