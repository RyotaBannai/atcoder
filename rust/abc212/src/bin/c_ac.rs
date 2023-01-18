use library::chmin;
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
 * Min Difference
 *
 * https://atcoder.jp/contests/abc212/tasks/abc212_c
 *
 * 近いのだけ見れば良いから O(N^2) で全探索しない方法を考える.
 * 一色たんに混ぜて、ソートして、隣り合う数が a,b のグループであれば差分を取ってmin を更新すると良い.
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut v = vec![];
    for x in a {
        v.push((x, 0));
    }
    for x in b {
        v.push((x, 1));
    }
    v.sort_unstable();

    let mut mi = std::usize::MAX;
    for i in 0..n + m - 1 {
        if v[i].1 == v[i + 1].1 {
            continue;
        }
        chmin!(mi, v[i + 1].0 - v[i].0);
    }

    println!("{}", mi);
}
