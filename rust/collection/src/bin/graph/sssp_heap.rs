/**
 * @cpg_dirspec sssp
 *
 * cpg run -p src/bin/graph/sssp_heap.rs
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
 * 単一始点最短経路（single source shortest path: SSSP）
 * : 根 s から任意の目的地 t への最短距離
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/ALDS1_12_C
 *
 * 最小全域木 != SSSP
 * 最小全域木: グラフ全体の辺の最小を求めたい（全体の最小化）
 * SSSP: 根 s からの各点 t への最小距離になるような全域木を求めたい
 *
 * tags: #ダイクストラ #dijkstra
 *
 * 木：閉路を持たないグラフ
 * グラフの全域木は一つであるとは限らない
 *
 * 最小全域木（minimum spanning tree, mst）：グラフの全域木の中で、辺の重みが最小の全域木
 *  -> 「閉路を作らず全ての頂点を通りつつ、辺の重みが最小になる」
 *
 * 最短経路木（shortest path spanning tree）: s を根として、s からグラフ G の全ての頂点への最短経路を包含する G の全域木
 *  -> 「特定の頂点を根として捉えて、そこから最小を求めるもの」
 *  単一始点最短経路（single source shortest path: SSSP）: 根からグラフ G の全ての頂点 d への最短経路
 *　全点対間最短経路（all pairs shortest path: APSP）: グラフ G の全ての頂点のペア間の最短経路
 *
 * ダイクストラ法
 * ・プリムのアルゴリズムと実装はほぼ同様
 * ・違いは、nd の最小を求める際に、プリムはどの頂点からのアクセスでも許容した場合の最小の辺を採用するのに対し、
 *    ダイクストラは通ってきた親までの重みに、自分の頂点へアクセスするために必要な重みを加えた時の最小の辺を採用する点
 * ・ダイクストラ法は、辺の重みが非負の場合のみ. 負値はある場合は、ベルマンフォード（Bellman Ford）/ ワーシャルフロイド（Warshall Floyd）を検討
 *
 * ダイクストラ法を優先度付きキューで処理する
 * ・アクセスしてる頂点から最短となる、かつ、まだアクセスしてない、かつ、移動できる頂点　を pq へ入れる
 * ・次の loop の回で pq から先頭を取り出す
 *
 */
use collection::utils::*;
use Color::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    White,
    Grey,
    Black,
}

// #[fastout]
fn main() {
    let v = read::<usize>()[0];
    let mut t = vec![vec![]; v]; // 0-index 連接リスト
    for a in 0..v {
        let line = read::<usize>();
        let e = line[1];
        if e != 0 {
            for j in 1..=e {
                let (b, w) = (line[j * 2], line[j * 2 + 1]);
                t[a].push((w, b)); //（有向）
            }
        }
    }

    let inf = 1 << 21;
    let mut c = vec![White; v];
    let mut d = vec![inf; v];
    d[0] = 0;

    // priority queue は優先度が一番高い要素を先頭に持っているだけで、全要素がソートされている訳ではない
    let mut x = BinaryHeap::new();
    x.push_rev((0, 0)); // (d, u)

    loop {
        // 候補の中から最小の辺を取り出す
        let y = x.pop_rev();

        if y.is_none() {
            // 全ての辺チェックした
            break;
        }
        let (ud, u) = y.unwrap();
        c[u as usize] = Black; // 今回で通過済み

        // 連接リストから取り出す
        for &(w, j) in &t[u] {
            let nd = ud + w;
            if c[j] != Black && d[j] > nd {
                d[j] = nd;
                x.push_rev((nd, j));
            }
        }
    }

    for (i, x) in d.iter().enumerate() {
        println!("{} {}", i, x);
    }
}
