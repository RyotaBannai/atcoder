use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
use ac_library_rs::modint::ModInt1000000007 as Mint;
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
use std::collections::VecDeque;

/**
 * Number of Shortest paths
 *
 * https://atcoder.jp/contests/abc211/tasks/abc211_d
 *
 * tags: #dfs #dp
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let mut g = vec![vec![]; n + 1];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }

    let mut dist = vec![1 << 20; n + 1];
    dist[1] = 0;
    let mut q = VecDeque::new();
    q.push_back(1);
    let mut xs = vec![];
    while let Some(u) = q.pop_front() {
        xs.push(u);
        for &y in g[u].iter() {
            if dist[y] == 1 << 20 {
                dist[y] = dist[u] + 1;
                q.push_back(y);
            }
        }
    }

    let mut dp = vec![Mint::new(0usize); n + 1];
    dp[1] += 1;
    for x in xs {
        for &u in g[x].iter() {
            if dist[x] + 1 == dist[u] {
                let c = dp[x];
                dp[u] += c;
            }
        }
    }

    println!("{}", dp[n]);
}
