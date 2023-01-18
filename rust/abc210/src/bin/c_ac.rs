use library::chmax;
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
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Colorful Candies
 *
 * https://atcoder.jp/contests/abc210/tasks/abc210_c
 *
 * 初めにk 個数えてから、一つずつずらす実装でも良い.
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        cs: [usize; n]
    }

    let mut ans = 0;
    let mut m = Map::new();
    let mut count = 0;
    for (i, &c) in cs.iter().enumerate() {
        *m.entry(c).or_insert(0) += 1;
        chmax!(ans, m.len());
        count += 1;
        if count == k {
            let del = cs[i + 1 - count];
            let e = m.entry(del).or_insert(0);
            *e -= 1;
            if *e == 0 {
                m.remove(&del);
            }
            count -= 1;
        }
    }
    println!("{}", ans);
}
