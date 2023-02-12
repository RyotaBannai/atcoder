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
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut uf = DisjointSet::new(n);
    for (a, b) in ab {
        if !uf.same(a, b) {
            uf.unite(a, b);
        }
    }

    let mut ans = 0;
    for u in 2..=n {
        if !uf.same(1, u) {
            uf.unite(1, u);
            ans += 1;
        }
    }

    println!("{}", ans);
}
