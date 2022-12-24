/**
 * @cpg_dirspec maximum_matching
 *
 * cpg run -p src/bin/graph/flow/maximum_matching.rs
 */
use std::cmp::{max, min};
use std::io;
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
 * 二部マッチング
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/7/GRL_7_A
 *
 * tags: #max_flow #最大流 #FordFullkerson #二部マッチング #二部グラフ #bipartite_graph #maximum_matching #最大マッチング
 *
 * https://algo-logic.info/ford-fullkerson/
 */
use library::{graph::max_flow::*, utils::read::*};

fn main() {
    // aoj
    let i = read::<usize>();
    let (x, y, e) = (i[0], i[1], i[2]);

    let mut g = Graph::new(x + y + 2);

    // 頂点ひとつが cap をもつのではなく、u-v 間で cap をもつ.
    // u-w 間は別の cap だから、u は v+w 分流すことができる.

    // aoj
    for _ in 0..e {
        let i = read::<usize>();
        let (u, v) = (i[0], i[1]);
        g.add_edge(u + 1, v + x + 1, 1);
    }

    for i in 1..=x {
        g.add_edge(0, i, 1);
    }

    for i in 0..=y {
        g.add_edge(x + i + 1, x + y + 1, 1);
    }

    let mut ford = FordFulkerson::new(g);
    let ans = ford.max_flow(0, x + y + 1);

    println!("{}", ans);
}
