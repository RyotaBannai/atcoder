/**
 * @cpg_dirspec apsp
 *
 * cpg run -p src/bin/graph/mst/apsp.rs
 */
// use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
use std::io;
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 全点対間最短経路（all pairs shortest path: APSP）: グラフ G の全ての頂点のペア間の最短経路
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/GRL_1_C
 *
 * tags: #全点対間最短経路 #APSP #ワーシャルフロイド #WarshallFloyd
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
 * ワーシャルフロイド（Warshall Floyd）
 * ・辺に重みの負の辺がなければダイクストラ法を|V|回行う（各頂点から sssp を流す）ことで解ける
 * ・負の閉路がない限り正しく動く（負の閉路がある場合無限にコストを小さくしてしまう）
 * ・負の閉路判定：任意の頂点 v から自分自身 v までの最短経路コストが負の場合
 *
 *
 * 参考
 * https://qiita.com/okaryo/items/8e6cd73f8a676b7a5d75
 */

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

// #[fastout]
fn main() {
    let r = read::<usize>();
    let (v, e) = (r[0], r[1]);

    // ワーシャルフロイド
    let inf = 1isize << 32;
    let mut d = vec![vec![inf; v]; v]; // 0-index で全頂点間分もつ
    for i in 0..v {
        for j in 0..v {
            if i == j {
                d[i][j] = 0;
            }
        }
    }

    for _ in 0..e {
        let line = read::<isize>();
        let (i, j, w) = (line[0], line[1], line[2]);
        d[i as usize][j as usize] = w; // 有効だけでok
    }

    // k が 0~v で先頭から処理されるから、1~5 の経路で k=2 経由した時には最小になったけど、その後、
    // 2~5 の最短経路が 3 経由することで更新される状況で、1~5 の経路も更新できるはずだが、最短経路になるのか. この場合、
    // 1~5 は k=2 を通る代わりに 3 を通るように整合性はある(?)（k=3 で 2 を通ることで最短となるのを知る前に, 1~3 を経由した 1~5 で k=2 よりも最短になることがすでにわかっているから最短経路を更新できる、はず（それまでの k=2 経由と今回の k=3 の min））
    for k in 0..v {
        for i in 0..v {
            if d[i][k] == inf {
                continue;
            }
            for j in 0..v {
                if d[k][j] == inf {
                    continue;
                }
                d[i][j] = min(d[i][j], d[i][k] + d[k][j]); // 頂点 k を経由しない場合, する場合の最小をとる
            }
        }
    }

    // print
    if (0..v).any(|i| d[i][i] < 0) {
        println!("NEGATIVE CYCLE");
        return;
    }

    for i in 0..v {
        for j in 0..v {
            if j != 0 {
                print!(" ");
            }

            if d[i][j] == inf {
                print!("INF");
            } else {
                print!("{}", d[i][j]);
            }
        }
        println!();
    }
}
