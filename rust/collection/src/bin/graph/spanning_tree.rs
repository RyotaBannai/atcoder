/**
 * @cpg_dirspec spanning_tree
 *
 * cpg run -p src/bin/graph/spanning_tree.rs
 */
// use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 最大流
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/GRL_6_A
 *
 * tags: #spanning_tree #全域木 #最小全域木 #prim
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
enum Color {
    White,
    Grey,
    Black,
}

// #[fastout]
fn main() {
    let r = read::<usize>();
    let (v, e) = (r[0], r[1]);
    let mut t = vec![vec![-1_isize; v]; v];
    // 辺の重みも考慮したいため、隣接行列を作る
    for i in 0..e {
        let a = read::<usize>();
        let (a, b, w) = (a[0], a[1], a[2] as isize);
        //（無向）
        t[a][b] = w;
        t[b][a] = w;
    }
    let inf = 2isize << 10;
    let mut c = vec![Color::White; v];
    let mut p = vec![-1; v];
    let mut d = vec![inf; v];
    d[0] = 0;

    let mut u: isize;
    let mut mi;

    loop {
        mi = inf;
        u = -1;

        // d に対して最小を見つける
        for i in 0..v {
            if c[i] != Color::Black && mi > d[i] {
                u = i as isize;
                mi = d[i];
            }
        }

        if u == -1 {
            break;
        }

        c[u as usize] = Color::Black;

        for j in 0..v {
            let nd = t[u as usize][j];
            if c[j] != Color::Black && nd != -1 && d[j] > nd {
                d[j] = nd;
                p[j] = u; // 通ってきた親の頂点番号
                c[j] = Color::Grey; // 探索中として Grey にしておく
            }
        }
    }

    let mut sum = 0usize;
    for i in 0..v {
        if p[i] != -1 {
            sum += t[i][p[i] as usize] as usize;
        }
    }

    println!("{}", sum);
}
