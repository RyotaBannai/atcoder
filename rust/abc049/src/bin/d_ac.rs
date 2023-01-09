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
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use library::structure::disjoint_set::*;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * D - 連結
 *
 * https://atcoder.jp/contests/abc049/tasks/arc065_b
 *
 * tags: #union_find
 *
 * 連結されている都市数>=2 が考えられるから、
 * 頂点u の道路、鉄道のroot をキーにしたmap にどちらも一致する都市数をカウント.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        l: usize,
        pq: [(usize, usize); k],
        rs: [(usize, usize); l],
    }

    let mut ufpq = DisjointSet::new(n);
    let mut ufrs = DisjointSet::new(n);
    for (p, q) in pq {
        if !ufpq.same(p, q) {
            ufpq.unite(p, q);
        }
    }

    for (r, s) in rs {
        if !ufrs.same(r, s) {
            ufrs.unite(r, s);
        }
    }

    let mut m = BTreeMap::<(usize, usize), usize>::new();
    for i in 1..=n {
        let ur1 = ufpq.find(i); // 道路
        let ur2 = ufrs.find(i); // 鉄道
        *m.entry((ur1, ur2)).or_insert(0) += 1;
    }

    for i in 1..=n {
        let ur1 = ufpq.find(i); // 道路
        let ur2 = ufrs.find(i); // 鉄道
        if let Some(x) = m.get(&(ur1, ur2)) {
            print!("{} ", x);
        }
    }
}
