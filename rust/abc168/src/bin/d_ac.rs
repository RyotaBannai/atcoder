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
use std::collections::{BinaryHeap, VecDeque};

/**
 * .. (Double Dots)
 *
 * https://atcoder.jp/contests/abc168/tasks/abc168_d
 *
 * tags: #bfs #bfs_base
 *
 * bfs をしながら、親を保持しておく.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let mut list = vec![vec![]; n + 1];
    for (a, b) in ab {
        list[a].push(b);
        list[b].push(a);
    }

    let mut q = VecDeque::new();
    q.push_back((1, 1, 0));
    let mut dist = vec![0; n + 1];
    let mut parent = vec![0; n + 1];
    while let Some((u, d, p)) = q.pop_front() {
        dist[u] = d;
        parent[u] = p;

        for &y in list[u].iter() {
            if dist[y] != 0 {
                continue;
            }
            dist[y] = d + 1;
            parent[y] = u;
            q.push_back((y, d + 1, u));
        }
    }
    // println!("{:?}", &dist);

    let mut ok = true;
    for &i in dist.iter().skip(1) {
        if dist[i] == 0 {
            // ある頂点から到達できない
            ok = false;
        }
    }
    if !ok {
        println!("No");
        return;
    }

    println!("Yes");
    for p in parent.iter().skip(2) {
        println!("{}", p);
    }
}
