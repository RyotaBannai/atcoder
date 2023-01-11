use itertools::Itertools;
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
use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::structure::disjoint_set::*;

/**
 * E - Destruction
 *
 * https://atcoder.jp/contests/abc218/tasks/abc218_e
 *
 * tags: #最小全域木 #minimum_spanning_tree #mst
 *
 * グラフが連結な限り辺を取り除いて良い=全域木なら良い
 * 辺の重みが最小の辺から順に使って、残りの辺の集合から最大になるように選んでいけば良い.
 * 負辺が残ることもあるから、0より大きい辺の総和を求める.
 *
 * mst をクラスカルで求めると O(ElogE), O(ElogV)
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, isize); m]
    }

    // 重みでソート
    let xs = abc
        .into_iter()
        .map(|(a, b, c)| (c, a, b))
        .sorted_by(|(a1, ..), (a2, ..)| a1.cmp(a2))
        .collect_vec();

    let mut rest = BTreeSet::from_iter(xs.iter().cloned());
    let mut ds = DisjointSet::new(n);
    for &x in &xs {
        let (_, a, b) = x;
        if !ds.same(a, b) {
            ds.unite(a, b);
            rest.remove(&x);
        }
    }

    let sum = rest
        .iter()
        .filter(|(a, _, _)| *a > 0)
        .map(|(a, _, _)| a)
        .sum::<isize>();
    println!("{}", sum);
}
