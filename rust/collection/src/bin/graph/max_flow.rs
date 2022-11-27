/**
 * @cpg_dirspec max_flow
 *
 * cpg run -p src/bin/graph/max_flow.rs
 */
use proconio::{fastout, input, marker::Chars};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 最大流
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/GRL_6_A
 *
 * tags: #max_flow #最大流 #FordFullkerson
 *
 * https://algo-logic.info/ford-fullkerson/
 */
use library::{graph::max_flow::*, utils::read::*};

#[fastout]
fn main() {
    input! {
      n: usize,
      e: usize,
      c: [(usize, usize, usize); e]
    }

    // aoj
    // let inp = read::<usize>();
    // let (n, e) = (inp[0], inp[1]);

    let mut g = Graph::new(n);

    // 頂点ひとつが cap をもつのではなく、u-v 間で cap をもつ.
    // u-w 間は別の cap だから、u は v+w 分流すことができる.
    for (u, v, cap) in c {
        g.add_edge(u, v, cap);
    }

    // aoj
    // for _ in 0..e {
    //     let inp = read::<usize>();
    //     let (u, v, cap) = (inp[0], inp[1], inp[2]);
    //     g.add_edge(u, v, cap);
    // }

    let mut ford = FordFulkerson::new(g);
    let ans = ford.max_flow(0, n - 1);

    println!("{}", ans);
}
