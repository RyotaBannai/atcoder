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
 * @cpg_dirspec sssp
 *
 * cpg run -p src/bin/graph/mst/sssp_heap.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use std::cmp::{max, min};
use std::collections::BinaryHeap;

/**
 * Number of Shortest paths
 *
 * https://atcoder.jp/contests/abc211/tasks/abc211_d
 *
 * tags: #ダイクストラ #dijkstra #SSSP #最短経路
 *
 */
use library::structure::rev_priority_queue::*;

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let mut t = vec![vec![]; n + 1];
    for (a, b) in ab {
        t[a].push((1, b));
        t[b].push((1, a));
    }

    let mo = 1_000_000_007;
    let inf = 1 << 21;
    let mut used = vec![false; n + 1];
    let mut d = vec![inf; n + 1];
    d[1] = 0;

    let mut x = BinaryHeap::new();
    x.push_rev((0, 1)); // (d, u)

    let mut sp_count = vec![(std::usize::MAX, 0); n + 1];
    sp_count[1] = (0, 1); // (dist, how many of path to vertex u)

    while let Some((ud, u)) = x.pop_rev() {
        if used[u] {
            continue;
        }
        used[u] = true;
        for &(w, j) in &t[u] {
            let nd = ud + w;
            if !used[j] {
                if d[j] == nd {
                    let (d, count) = sp_count[j];
                    sp_count[j] = (d, (count + sp_count[u].1) % mo);
                }
                if d[j] > nd {
                    sp_count[j] = (nd, sp_count[u].1);
                    d[j] = nd;
                    x.push_rev((nd, j));
                }
            }
        }
    }

    println!("{}", sp_count[n].1);
}
