use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 078 - Easy Graph Problem（★2）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bz
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(usize, usize); m]
    }

    let mut v = vec![Set::new(); n];
    for (a, b) in es {
        // 単純無効グラフ
        v[a - 1].insert(b - 1);
        v[b - 1].insert(a - 1);
    }

    let mut ans = 0;
    for (i, s) in v.iter().enumerate() {
        let p = s.iter().collect::<Vec<_>>().lower_bound(&&i);
        if p == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
