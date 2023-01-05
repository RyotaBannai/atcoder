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
use library::structure::disjoint_set::*;

/**
 * Decayed Bridges
 *
 * https://atcoder.jp/contests/abc120/tasks/abc120_d
 *
 * tags: #union_find #余事象 #組合せ
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut uf = WeightedDisjointSet::new(n);

    // M 本の橋が全て無いとき、各島は独立しているから連結成分数はN 個.
    // その状態から端を逆順でつなげていけば再現できる.

    let mut count = n * (n - 1) / 2;
    let mut hist = vec![count];
    for &(a, b) in ab.iter().rev() {
        if uf.same(a, b) {
            // 連結されている組に橋をつなげても組合せ数は変化しない
            hist.push(count);
            continue;
        }

        // 全組合せ数から余事象を引いていく
        // 余事象は同じ連結成分内の組合せ数だから、連結できる度にそれを引いていく.
        // すでに前の処理で、前の連結成分内での組合せ数を引いているから、それらを先に戻して
        // より大きな連結成分内での組合せ数を余事象として引くと良い.
        let mut sub = 0;
        let ra = uf.find(a);
        let ra_s = uf.size[ra];
        if ra_s != 1 {
            sub += ra_s * (ra_s - 1) / 2;
        }
        let rb = uf.find(b);
        let rb_s = uf.size[rb];
        if rb_s != 1 {
            sub += rb_s * (rb_s - 1) / 2;
        }
        let ttl = ra_s + rb_s;
        count += sub;
        count -= ttl * (ttl - 1) / 2;
        uf.merge(a, b, 1);
        hist.push(count);
    }

    // println!("{:?}", &hist);

    for x in hist.iter().rev().skip(1) {
        println!("{}", x);
    }
}
