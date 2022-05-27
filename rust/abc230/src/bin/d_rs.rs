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
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Destroyer Takahashi
 *
 * https://atcoder.jp/contests/abc230/tasks/abc230_d
 *
 * tags: #区間スケジューリング問題 #Islands War
 *
 * 類似:
 * https://atcoder.jp/contests/abc103/tasks/abc103_d
 *
 * https://swim-run.net/1193/
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        mut s: [(usize, usize); n]
    }

    s.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut cnt = 1;
    let mut r = s[0].1;

    for x in s.iter().skip(1) {
        if r + d - 1 < x.0 {
            r = x.1;
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
