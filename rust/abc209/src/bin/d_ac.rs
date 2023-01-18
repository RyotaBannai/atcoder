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
use std::usize::MAX;

/**
 * Collision
 *
 * https://atcoder.jp/contests/abc209/tasks/abc209_d
 *
 * tags: #bfs
 *
 * 幅探索で根を１とした時の各頂点への最短距離を求めた後に、
 * クエリーの頂点間の距離をO(1) で計算する.
 * 頂点間距離が偶数の時は頂点、奇数の時は辺で交わることがわかる.
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize,usize); n-1],
        cd: [(usize,usize); q]
    }
    let mut list = vec![vec![]; n + 1];
    for (a, b) in ab {
        list[a].push(b);
        list[b].push(a);
    }
    let mut dist = vec![MAX; n + 1];
    let mut qu = VecDeque::new();
    dist[0] = 0;
    qu.push_back((1, 0));
    while let Some((u, p)) = qu.pop_front() {
        if dist[u] != MAX {
            continue;
        }
        dist[u] = dist[p] + 1;
        for &y in list[u].iter() {
            if dist[y] != MAX {
                continue;
            }
            qu.push_back((y, u));
        }
    }

    // println!("{:?}", &dist);
    let calc = |m| if m % 2 == 0 { "Town" } else { "Road" };
    for (c, d) in cd {
        let dif = if dist[c] > dist[d] {
            dist[c] - dist[d]
        } else {
            dist[d] - dist[c]
        };
        println!("{}", calc(dif));
    }
}
