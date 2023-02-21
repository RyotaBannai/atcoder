use library::chmax;
use proconio::{fastout, input, marker::Chars};
use superslice::Ext;
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
 * Tsundoku
 *
 * https://atcoder.jp/contests/abc172/tasks/abc172_c
 *
 * tags: #二分探索 #単調増加することを利用
 *
 * 一方を何冊読むか固定して、他方の読める数を高速に求めるとよい.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut asum = vec![0; n + 1];
    for i in 0..n {
        asum[i + 1] += asum[i] + a[i];
    }

    let mut bsum = vec![0; m + 1];
    for i in 0..m {
        bsum[i + 1] += bsum[i] + b[i];
    }

    let mut ma = 0;
    for i in 0..=n {
        let t = asum[i];
        if t > k {
            // a のスタックを読むだけでk を超えてしまった
            break;
        }
        let pos = bsum.upper_bound(&(k - t));
        let x = i + pos - 1;
        chmax!(ma, x);
    }
    println!("{}", ma);
}
