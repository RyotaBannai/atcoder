/**
 * @cpg_dirspec spanning_tree
 *
 * cpg run -p src/bin/graph/spanning_tree_kruskal.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
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
 * 最小全域木
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/GRL_2_A
 *
 * tags: #spanning_tree #全域木 #最小全域木 #kruskal
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
 * クラスカル法
 * ・全辺を昇順ソートする
 * ・ソート順に最小全域木 T にその頂点が含まれていない場合は T に新たに追加する
 * ・含まれているかどうかの判定には、UninonFind を用いると高速
 *
 *
 * 参考
 * https://algo-logic.info/kruskal-mst/
 */
use collection::{structure::disjoint_set::*, utils::read::*};

// #[fastout]
fn main() {
    let r = read::<usize>();
    let (v, e) = (r[0], r[1]);

    let mut xs = vec![];
    for _ in 0..e {
        let a = read::<usize>();
        let (a, b, w) = (a[0], a[1], a[2] as isize);
        xs.push((w, a, b)); // 有効だけでok
    }

    xs.sort_unstable_by(|(a, ..), (b, ..)| a.cmp(b)); // 重みでソート

    // let mut p = vec![-1isize; v];
    let mut path = vec![];

    let mut sum = 0usize;
    let mut ds = DisjointSet::new(v);
    for x in xs {
        let (w, a, b) = x;
        if !ds.same(a, b) {
            ds.unite(a, b);
            sum += w as usize;
            path.push(x);

            // p[b] = a as isize; // prim のように通った親がわからないから、これができない（p の更新が被る）
        }
    }

    println!("{}", sum);
}
