use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use library::structure::rev_priority_queue::*;
use std::collections::{BinaryHeap, VecDeque};
use std::usize::MAX;

/**
 * Train
 *
 * https://atcoder.jp/contests/abc192/tasks/abc192_e
 *
 *
 * tags: #ダイクストラ #dijkstra
 *
 * 注意:
 * 多重辺の可能性もあるから辺の重みを一意に決定しないこと.
 *
 *
 */

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        y: usize,
        abtk: [(usize,usize,usize,usize); m]
    }
    let mut list = vec![vec![]; n + 1]; // 1-index 無向グラフ
    for (a, b, t, k) in abtk {
        list[a].push((b, t, k));
        list[b].push((a, t, k));
    }
    let mut d = vec![MAX; n + 1]; // 頂点分用意 1-index
    d[x] = 0;

    let mut q = BinaryHeap::new();
    q.push_rev((0, x));

    while let Some((tt, u)) = q.pop_rev() {
        if d[u] < tt {
            continue;
        }

        for &(y, nt, nk) in list[u].iter() {
            let wait = (nk - tt % nk) % nk; // nk の倍数でないと出発できない
            let nd = tt + wait + nt;
            if nd < d[y] {
                // y に向かう時の最短時間を更新できるならば
                q.push_rev((nd, y));
                d[y] = nd;
            }
        }
    }

    if d[y] == MAX {
        println!("-1");
    } else {
        println!("{}", d[y]);
    }
}
