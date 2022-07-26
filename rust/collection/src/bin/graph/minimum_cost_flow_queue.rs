/**
 * @cpg_dirspec minimum_cost_flow
 *
 * cpg run -p src/bin/graph/minimum_cost_flow_queue.rs
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
use std::collections::BinaryHeap;
use std::io;

/**
 * 最小費用流（minimum cost flow）
 * ：フローネットワック上で、各辺にコストがある場合で、フロー f を流した時の最小コストを求めたい
 *
 * https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_B&lang=ja
 *
 * tags: #ダイクストラ #dijkstra #最小費用流 #minimum_cost_flow
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
 * 全点対間最短経路（all pairs shortest path: APSP）: グラフ G の全ての頂点のペア間の最短経路
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
 * sssp 実装例
 * src/bin/graph/sssp_heap.rs
 *
 *
 * ダイクストラ + 最小費用流（minimum cost flow）
 * ・ポテンシャル（h）を使う
 *  ・d[from] + e.cost + h[from] - h[e.to]
 *  ・h は毎回流量を流すごとに加算されていく
 *  ・前回の最短距離で更新されたポテンシャルで引く = より大きな数で引くため, 最短距離として採用される(nd)
 *  ・コスト計算時には、d では前回使用した分のポテンシャルを引いているため、今回分をポテンシャル（h[t]）に加算して計算
 *
 * 参考
 * https://qiita.com/drken/items/e805e3f514acceb87602#4-2-%E6%9C%80%E5%B0%8F%E8%B2%BB%E7%94%A8%E6%B5%81%E5%95%8F%E9%A1%8C%E3%82%92%E8%A7%A3%E3%81%8F%E3%82%A2%E3%83%AB%E3%82%B4%E3%83%AA%E3%82%BA%E3%83%A0
 *
*/

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Edge {
    rev: usize,
    from: usize,
    to: usize,
    cap: isize,
    cost: isize,
}
impl Edge {
    fn new(rev: usize, from: usize, to: usize, cap: isize, cost: isize) -> Self {
        Self {
            rev,
            from,
            to,
            cap,
            cost,
        }
    }
}

// #[fastout]
fn main() {
    let m = read::<usize>();
    let (v, e, mut f) = (m[0], m[1], m[2] as isize);
    let mut g = vec![vec![]; v]; // 全辺
    for _ in 0..e {
        let line = read::<usize>();
        let (from, to, cap, cost) = (line[0], line[1], line[2] as isize, line[3] as isize);
        let fe = Edge::new(g[to].len(), from, to, cap, cost);
        let te = Edge::new(g[from].len(), to, from, 0, -cost);
        g[from].push(fe);
        g[to].push(te);
    }

    let inf = 1 << 21;
    let mut d; // 最短距離（最小距離）
    let mut prev = vec![0; v]; // 直前の頂点（親）
    let mut pree = vec![0; v]; // 直前の辺（親と子の辺）
    let mut h = vec![0; v]; // ポテンシャル // 初めにすべて 0 で初期化

    let mut res = 0;
    let t = v - 1; // 終点
    let s = 0; // 始点

    while f > 0 {
        let mut x = BinaryHeap::new(); // first: 最短距離、second: 頂点番号
        x.push((0, s));
        d = vec![inf; v];
        d[s] = 0;

        // ダイクストラ法なので負の閉路は未考慮
        while !x.is_empty() {
            let (p, from) = x.pop().unwrap();
            if d[from] < p {
                // 最短経路じゃなければ処理しない
                // 通過済かどうかのチェックしなくて良い
                continue;
            }

            // inf の考慮は不要？ -> queue から取り出している == 通過できる頂点
            // if d[from] == inf {
            // continue;
            // }

            for (i, e) in g[from].iter().enumerate() {
                if e.cap <= 0 {
                    continue;
                }
                let nd = d[from] + e.cost + h[from] - h[e.to]; // nd 負は取らない
                if nd < d[e.to] {
                    // 残量があって最短経路
                    d[e.to] = nd;
                    prev[e.to] = from;
                    pree[e.to] = i;
                    x.push((nd, e.to));
                }
            }
        }

        // 終点 t までたどりつけない
        if d[t] == inf {
            println!("-1");
            return;
        }

        for i in 0..v {
            h[i] += d[i];
        }

        // 現時点で残りの流したい流量 f を終点から始点まで通るパスのそれぞれの頂点の cap の最小をとる（それがソースから流せる最大量 f）
        let mut mi = f;
        let mut u = t;
        while u != s {
            mi = mi.min(g[prev[u]][pree[u]].cap);
            u = prev[u];
        }

        f -= mi; // mi が今回流せる流量. 残りは次回

        // e.g. test case 2
        // d 5
        // h 5
        // mi 2

        // d 3
        // h 8
        // mi 1
        // println!("d {}", d[t]);
        // println!("h {}", h[t]);
        // println!("mi {}", mi);
        // println!("{:?}", &prev);
        res += mi * h[t]; // 終端までのコスト h[t] * 流量

        u = t;
        while u != s {
            let e = &mut g[prev[u]][pree[u]]; // 可変参照
            let rev = e.rev;
            e.cap -= mi;
            g[u][rev].cap += mi;
            u = prev[u];
        }
    }

    println!("{}", res);
}
