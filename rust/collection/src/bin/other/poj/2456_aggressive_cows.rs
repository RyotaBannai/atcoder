/**
 * @cpg_dirspec 2456_aggressive_cows
 *
 * cpg run -p src/bin/other/poj/2456_aggressive_cows.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max,
//     min,
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
use library::utils::read::read;

/**
 * Aggressive cows
 *
 * http://poj.org/problem?id=2456
 *
 * sample のテストケースしか試してない.
 * lr の区間の調整しないとだめかも.
 *
 */

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, c) = (a[0], a[1]);
    let mut xs = vec![];
    for _ in 0..n {
        xs.push(read::<usize>()[0]);
    }
    xs.sort_unstable(); // stall の位置をソート

    let mut l = 0usize;
    let mut r = *xs.iter().max().unwrap();

    while r - l > 1 {
        let dist = (r + l) / 2;
        let mut count = 0;
        let mut acc = 0;
        let mut prev = 0;
        for i in 0..n {
            let nx = xs[i];
            acc += nx - prev;
            prev = nx;
            if acc >= dist {
                count += 1;
                acc = 0; // リセット
            }
        }
        if count >= c {
            // count が多い=stall の間隔が狭いからたくさんcow が入る.= stall の間隔dist を広げられるから、下限を上げる.
            l = dist;
        } else {
            r = dist;
        }
    }
    println!("{}", r);
}
