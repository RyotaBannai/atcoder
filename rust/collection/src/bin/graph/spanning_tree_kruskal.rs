/**
 * @cpg_dirspec spanning_tree
 *
 * cpg run -p src/bin/graph/spanning_tree_kruskal.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
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

struct DisjointSet {
    rank: Vec<usize>,
    p: Vec<usize>,
}
impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut p = vec![0; n + 1];
        let mut rank = vec![0; n + 1];
        for i in 1..=n {
            p[i] = i;
            rank[i] = 0;
        }
        Self { rank, p }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn find(&mut self, x: usize) -> usize {
        if x != self.p[x] {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let p1 = self.find(x);
        let p2 = self.find(y);
        self.link(p1, p2);
    }
    fn link(&mut self, x: usize, y: usize) {
        if self.rank[x] > self.rank[y] {
            self.p[y] = x; // ランクが大き方につける
        } else {
            self.p[x] = y;
            if self.rank[x] == self.rank[y] {
                // rank 更新前は同じ可能性がある
                self.rank[y] += 1;
            }
        }
    }
}

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
