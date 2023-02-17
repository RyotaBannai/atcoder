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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
use library::structure::rev_priority_queue::*;
use library::utils::ord_tools::pair_sort;
use std::collections::{BinaryHeap, VecDeque};
use std::usize::MAX;

/**
 * Line++
 *
 * https://atcoder.jp/contests/abc160/standings/virtual
 *
 * tags: #dikjstra #ダイクストラ #ダイクストラ_base #dijkstra_base
 *
 *
 * 各頂点から最短経路を求めた結果を集計する.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize
    }
    let mut list = vec![vec![]; n]; // 0-index 無向グラフ
    for i in 0..n - 1 {
        list[i].push(i + 1);
        list[i + 1].push(i);
    }
    list[x - 1].push(y - 1);
    list[y - 1].push(x - 1);

    let mut count = vec![Set::new(); n]; // 距離がkのユニークな組み合わせ

    for i in 0..n {
        let mut d = vec![MAX; n];
        d[i] = 0;

        let mut q = BinaryHeap::new();
        q.push_rev((0, i));

        while let Some((t, u)) = q.pop_rev() {
            if d[u] < t {
                continue;
            }
            for &y in list[u].iter() {
                let nd = t + 1;
                if nd < d[y] {
                    q.push_rev((nd, y));
                    d[y] = nd;
                }
            }
        }

        for to in 0..n {
            let k = d[to];
            if i == to {
                // 頂点が同じならpass
                continue;
            }
            count[k].insert(pair_sort((i, to)));
        }
    }

    for x in count.iter().skip(1) {
        println!("{}", x.len());
    }
}
