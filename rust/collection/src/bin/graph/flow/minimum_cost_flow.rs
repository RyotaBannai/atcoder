/**
 * @cpg_dirspec minimum_cost_flow
 *
 * cpg run -p src/bin/graph/flow/minimum_cost_flow.rs
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
use std::cmp::{max, min};
use std::io;

/**
 * 最小費用流（minimum cost flow）
 * ：フローネットワック上で、各辺にコストがある場合で、フロー f を流した時の最小コストを求めたい
 *
 * https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_B&lang=ja
 *
 * tags: #ベルマンフォード #bellman_ford #最小費用流 #minimum_cost_flow
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
 * ベルマンフォード（Bellman Ford）法
 * ・最短距離が更新されなくなるか、|V| 回目の更新が終わるまで以下を繰り返す
 *    全ての辺に対して、「d[v] = min{ d[u] + ( u から v への距離 ) }」という更新式を利用して最短距離を更新する
 * ・|V-1| 回以内の更新で終了すれば負の閉路は存在しない。|V| 回まで更新が続けば負の閉路が存在する
 *
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

    // println!("{:?}", &g);

    let inf = 1 << 21;
    let mut d: Vec<isize>; // 最短距離（最小距離）
    let mut prev = vec![0; v]; // 直前の頂点（親）
    let mut pree = vec![0; v]; // 直前の辺（親と子の辺）

    // ベルマンフォードで最短経路木を求める
    let mut res = 0;
    let t = v - 1; // 終点
    let s = 0;

    while f > 0 {
        d = vec![inf; v]; // cap は前の回で更新されているけど、次回の d は初期化
        d[s] = 0; // 始点 s だけ 0 で初期化することで処理がそこから始まる.(!=inf)

        // 負の閉路は未考慮
        let mut update = true;
        while update {
            update = false;
            for (from, xs) in g.iter().enumerate() {
                if d[from] == inf {
                    continue;
                }
                for (i, e) in xs.iter().enumerate() {
                    if e.cap <= 0 {
                        continue;
                    }
                    let nd = d[from] + e.cost;
                    if nd < d[e.to] {
                        // 残量があって最短経路
                        d[e.to] = nd;
                        prev[e.to] = from;
                        pree[e.to] = i;
                        update = true;
                    }
                }
            }
        }

        // 終点 t までたどりつけない
        if d[t] == inf {
            println!("-1");
            return;
        }

        // 現時点で残りの流したい流量 f を終点から始点まで通るパスのそれぞれの頂点の cap の最小をとる（それがソースから流せる最大量 f）
        let mut mi = f;
        let mut u = t;
        while u != s {
            mi = min(mi, g[prev[u]][pree[u]].cap);
            u = prev[u];
        }

        f -= mi; // mi が今回流せる流量. 残りは次回
        res += mi * d[t]; // なんで t? 終端までのコスト d[t] * 流量

        u = t;
        while u != s {
            let e = &mut g[prev[u]][pree[u]]; // Copy でなく可変参照. Copy trait がついてると Copy 代入されていることに気づかない..
            let rev = e.rev;
            e.cap -= mi;
            g[u][rev].cap += mi;
            u = prev[u];
        }
    }

    println!("{}", res);
}
