use proconio::{fastout, input, marker::Chars};
use std::usize::MAX;
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
 * D - People on a Line
 *
 * tags: #weighted_union_find #重み付きunion_find
 *
 * https://atcoder.jp/contests/abc087/tasks/arc090_b
 *
 * 参考
 * https://qiita.com/drken/items/cce6fc5c579051e64fab
 */
use abc087::structure::disjoint_set::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(usize, usize, isize); m]
    }

    let mut ds = WeightedDisjointSet::new(n);
    for (l, r, w) in es {
        if ds.same(l, r) {
            // r の人は、l より d だけ右にいる
            if ds.diff(l, r) != w {
                println!("No");
                return;
            }
        } else {
            ds.merge(l, r, w);
        }
    }

    println!("Yes");
}
