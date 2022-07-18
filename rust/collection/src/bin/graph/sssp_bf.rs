/**
 * @cpg_dirspec sssp_neg
 *
 * cpg run -p src/bin/graph/sssp_bf.rs
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

/**
* 単一始点最短経路（single source shortest path: SSSP）
* : 根 s から任意の目的地 t への最短距離
*
* https://onlinejudge.u-aizu.ac.jp/problems/GRL_1_B
*
* 最小全域木 != SSSP
* 最小全域木: グラフ全体の辺の最小を求めたい（全体の最小化）
* SSSP: 根 s からの各点 t への最小距離になるような全域木を求めたい
*
* tags: #ベルマンフォード #bellman_ford
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
* ベルマンフォード（Bellman Ford）法
* ・最短距離が更新されなくなるか、|V| 回目の更新が終わるまで以下を繰り返す
*    全ての辺に対して、「d[v] = min{ d[u] + ( u から v への距離 ) }」という更新式を利用して最短距離を更新する
* ・|V-1| 回以内の更新で終了すれば負の閉路は存在しない。|V| 回まで更新が続けば負の閉路が存在する
*
* 参考
* https://algo-logic.info/bellman-ford/
*/
use std::io;

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
    cost: isize,
}
impl Edge {
    fn new(from: usize, to: usize, cost: isize) -> Self {
        Self { from, to, cost }
    }
}

// #[fastout]
fn main() {
    let m = read::<usize>();
    let (v, e, r) = (m[0], m[1], m[2]);
    let mut xs = vec![]; // 全辺
    for _ in 0..e {
        let line = read::<isize>();
        let (a, b, w) = (line[0], line[1], line[2]);
        xs.push(Edge::new(a as usize, b as usize, w)); // 有向
    }

    let inf = 1 << 21;
    let mut d = vec![inf; v];
    d[r] = 0; // 始点 s だけ 0 で初期化することで処理がそこから始まる.(!=inf)

    let mut count = 0;
    while count < v {
        let mut end = true;
        for &x in &xs {
            let nd = d[x.from] + x.cost;
            // x.to にはパスが存在すれば、inf 以外のコストが入っている.
            if d[x.from] != inf && nd < d[x.to] {
                d[x.to] = nd;
                end = false;
            }
        }
        if end {
            break;
        }
        count += 1;
    }

    // print
    if count == v {
        println!("NEGATIVE CYCLE");
        return;
    }
    for &x in d.iter() {
        if x == inf {
            println!("INF");
        } else {
            println!("{}", x);
        }
    }
}
