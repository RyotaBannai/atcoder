use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
use maplit::hashmap;
use std::collections::{BTreeMap, BTreeSet, HashMap};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * E - Crystal Switches
 *
 * https://atcoder.jp/contests/abc277/tasks/abc277_e
 *
 *
 * 単一始点最短経路（single source shortest path: SSSP）
 * : 根 s から任意の目的地 t への最短距離
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/ALDS1_12_B
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
 */
use abc277::utils::*;

#[fastout]
fn main() {
    input! {
        n: usize, // 2<=
        m: usize, // 1<=
        k: usize, // 0<=k
        es: [(usize, usize, usize); m], // 無効グラフ, 通れる・通れないの初期状態
        s: [usize; k], // スイッチがおいてある頂点 1<=
    }
    let mut swi = vec![false; n + 1];
    for x in s {
        swi[x] = true;
    }
    let mut map = vec![vec![]; n + 1];
    for &e in &es {
        let (a, b, s) = e;
        map[a].push((b, s));
        map[b].push((a, s));
    }

    let inf = std::i32::MAX / 10;
    let mut d = vec![vec![inf; 2]; n + 1]; // 距離 スイッチの切り替えの状態（偶奇）で分ける
    d[1][1] = 0;

    let mut x = VecDeque::new();
    x.push_back((1, 1));

    // 候補の中から最小コストで入っている頂点を取り出す
    while let Some((onoff, u)) = x.pop_front() {
        let ud = d[u][onoff];

        let no = onoff ^ 1;
        if swi[u] && ud < d[u][no] {
            d[u][no] = ud;
            x.push_front((no, u));
        }

        let nd = ud + 1;
        for &(chi, p) in map[u].iter() {
            if p == onoff && nd < d[chi][onoff] {
                d[chi][onoff] = nd;
                x.push_back((onoff, chi));
            }
        }
    }

    let mut ans = inf;
    for &x in d[n].iter() {
        ans = ans.min(x);
    }
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
