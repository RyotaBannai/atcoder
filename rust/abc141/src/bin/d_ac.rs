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
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * Powerful Discount Tickets
 *
 * https://atcoder.jp/contests/abc141/tasks/abc141_d
 *
 * tags: #heap
 *
 * チケット数が 10^5 だから愚直に実装する
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut m: usize,
        a: [usize;n]
    }
    let mut q = BinaryHeap::new();
    for x in a {
        q.push(x);
    }
    while let Some(x) = q.pop() {
        if x == 0 {
            break;
        }
        q.push(x / 2);
        m -= 1;
        if m == 0 {
            break;
        }
    }
    let mut ans = 0;
    while let Some(x) = q.pop() {
        ans += x;
    }
    println!("{}", ans);
}
