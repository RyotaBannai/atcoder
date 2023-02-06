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
 * Don’t be cycle
 *
 * https://atcoder.jp/contests/abc288/tasks/abc288_c
 *
 * tags: #union_find
 *
 * 順に頂点をつないでい行って、閉路になる辺をつなげないでそれをカウントするだけ.
 *
 *
 */
use library::structure::disjoint_set::*;
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut ans = 0;
    let mut uf = DisjointSet::new(n);
    for (a, b) in ab {
        if uf.same(a, b) {
            ans += 1;
        } else {
            uf.unite(a, b);
        }
    }

    println!("{}", ans);
}
