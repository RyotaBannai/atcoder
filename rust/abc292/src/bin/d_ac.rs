use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Unicyclic Components
 *
 * https://atcoder.jp/contests/abc292/tasks/abc292_d
 *
 * tags: #連結成分 #連結 #部分グラフ
 *
 * 連結成分はサイクルになっている必要はなくて、頂点間に張られている辺を好きなだけ使って行き来できる部分グラフであれば
 * 「連結」であると言える
 *
 * 連結成分：連結な部分グラフのうち、そのグラフを含んだより大きい連結な部分グラフが存在しないもの
 * これは辺で繋がっている頂点同士であれば同じ連結成分とするということで、その連結成分に含まれているより小さな部分グラフは考えないということ.
 *
 */
use library::structure::disjoint_set::*;
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m]
    }
    let mut dsu = DisjointSet::new(n);
    let mut vs = vec![0; n + 1];
    let mut es = vec![0; n + 1];
    for &(x, y) in &xy {
        dsu.unite(x, y);
    }
    for u in 1..=n {
        vs[dsu.find(u)] += 1;
    }
    for i in 0..m {
        es[dsu.find(xy[i].0)] += 1;
    }
    if vs == es {
        println!("Yes");
    } else {
        println!("No");
    }
}
