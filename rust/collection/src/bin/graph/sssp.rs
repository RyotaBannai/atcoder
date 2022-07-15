/**
 * @cpg_dirspec sssp
 *
 * cpg run -p src/bin/graph/sssp.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
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
 * ・ダイクストラ法は、辺の重みが非負の場合のみ. 負値はある場合は、ベルマンフォード（Bellman Ford）/ わーシャルフロイド（Warshall Floyd）を検討
 *
 */
use std::io;
use Color::*;

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    White,
    Grey,
    Black,
}

// #[fastout]
fn main() {
    let v = read::<usize>()[0];
    let mut t = vec![vec![-1_isize; v]; v]; // 0-index
    for a in 0..v {
        let line = read::<usize>();
        let e = line[1];
        if e != 0 {
            for j in 1..=e {
                let (b, w) = (line[j * 2], line[j * 2 + 1]);
                t[a][b] = w as isize; //（有向）

                // t[b][a] = w as isize;
            }
        }
    }

    let inf = 1isize << 21;
    let mut c = vec![White; v];
    let mut d = vec![inf; v];
    d[0] = 0;
    c[0] = Grey;

    let mut u: isize;
    let mut mi;

    let mut s = Set::new();
    for i in 0..v {
        s.insert(i);
    }
    loop {
        mi = inf;
        u = -1;

        // すでに通った Grey の中から最小の d を見つける(実際には Grey の判定はしていないが、White は Inf であり、最小の d にはならない)
        for &i in &s {
            if mi > d[i] {
                u = i as isize;
                mi = d[i];
            }
        }

        if u == -1 {
            // 全ての辺チェックした
            break;
        }

        c[u as usize] = Black;
        s.remove(&(u as usize));

        for j in 0..v {
            // MST との違い MST は最小の辺(d[j] > nd, どの頂点からのアクセスでも良い)を見るが、SSSP は親からの距離で見る
            let w = t[u as usize][j];
            let nd = d[u as usize] + w;
            if c[j] != Black && w != -1 && d[j] > nd {
                d[j] = nd;
                // c[j] = Grey; // 探索中として Grey にしておいてもいい
            }
        }
    }

    for (i, x) in d.iter().enumerate() {
        println!("{} {}", i, x);
    }
}
