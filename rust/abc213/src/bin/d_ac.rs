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

use library::graph::{euler_tour::*, vertex::*};

/**
 * D - Takahashi Tour
 *
 * https://atcoder.jp/contests/abc213/tasks/abc213_d
 *
 * tags: #オイラーツアー #euler_tour
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        st: [(usize, usize); n-1]
    }

    let mut list = vec![vec![]; n + 1];
    for (s, t) in st {
        list[s].push(Vertex::new(s, t, 1));
        list[t].push(Vertex::new(t, s, 1));
    }
    for xs in list.iter_mut() {
        xs.sort_unstable_by(|a, b| a.to.cmp(&b.to));
    }

    let et = euler_tour(Vertex::new(0, 1, 1), &list);
    for x in et.visit {
        print!("{} ", x);
    }
    println!();
}
