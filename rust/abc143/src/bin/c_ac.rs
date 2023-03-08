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
use std::collections::{BinaryHeap, VecDeque};

/**
 * Slimes
 *
 * https://atcoder.jp/contests/abc143/tasks/abc143_c
 *
 * tags: #heap
 *
 * 隣あうものを再帰的に連結して行った時の残りを数える問題.
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut q = VecDeque::new();

    for c in s {
        if let Some(u) = q.pop_back() {
            if u == c {
                // 一つにくっつけて入れる
                q.push_back(c);
            } else {
                // 異なる色なら
                // 順番に入れ直す
                q.push_back(u);
                q.push_back(c);
            }
        } else {
            q.push_back(c);
        }
    }
    println!("{}", q.len());
}
