/**
 * @cpg_dirspec weighted_disjoint_set
 *
 * cpg run -p src/bin/query/weighted_disjoint_set.rs
 */
// use proconio::{fastout, input, marker::Chars};
use std::io;
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
 * https://onlinejudge.u-aizu.ac.jp/problems/DSL_1_B
 *
 * tags: #weighted_union_find #重み付きunion_find
 *
 *
 * 参考
 * ・https://qiita.com/drken/items/cce6fc5c579051e64fab
 * ・/Users/ryotabannai/Documents/dev/atcoder/atcoder/rust/abc087/src/bin/d_ac.rs
 */
use collection::{structure::disjoint_set::*, utils::read::*};

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, q) = (a[0], a[1]);

    let mut ds = WeightedDisjointSet::new(n);
    for _ in 0..q {
        let b = read::<usize>();
        if b[0] == 0 {
            let (x, y, z) = (b[1], b[2], b[3] as isize);
            ds.merge(x, y, z);
        } else {
            let (x, y) = (b[1], b[2]);
            if ds.same(x, y) {
                println!("{}", ds.diff(x, y));
            } else {
                println!("?");
            }
        }
    }
}
