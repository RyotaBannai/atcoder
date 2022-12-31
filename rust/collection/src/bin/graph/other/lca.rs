/**
 * @cpg_dirspec lca
 *
 * cpg run -p src/bin/graph/other/lca.rs
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
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::{
    graph::{euler_tour::*, vertex::*},
    query::seg_tree::*,
    utils::read::*,
    *,
};

/**
 * LCA: Lowest Common Ancestor
 * solved with Euler Tour + Segment Tree
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/GRL_5_C
 *
 */

// #[fastout]
fn main() {
    let n = read::<usize>()[0];
    let mut list = vec![vec![]; n + 1];
    for i in 0..n {
        // i 番目の親子関係を取り出す.
        let b = read::<usize>();
        let k = b[0];
        for j in 1..=k {
            // i-index で管理
            list[i + 1].push(Vertex::new(i + 1, b[j] + 1, 1));
            list[b[j] + 1].push(Vertex::new(b[j] + 1, i + 1, 1));
        }
    }

    // read query
    let q = read::<usize>()[0];
    let mut qs = vec![];
    for _ in 0..q {
        let b = read::<usize>();
        // i-index で管理
        qs.push((b[0] + 1, b[1] + 1));
    }

    // for xs in list.iter() {
    //     println!("{:?}", &xs);
    // }

    let et = euler_tour(Vertex::new(0, 1, 1), &list);

    // RMQ tuple
    let mut seg = LazySegTree::new(
        et.visit.len(),
        ((1 << 31) - 1, 0),
        ((1 << 31) - 1, 0),
        (1 << 31) - 1,
        0,
        |a: (usize, usize), b: (usize, usize)| a.min(b), // min
        |a: (usize, usize), _: isize, _: usize| a,
        |a: (usize, usize), _: isize, _: usize| a,
        |_: isize, y: isize| y,
        |_: isize, y: isize| y,
        |a: (usize, usize), x: (usize, usize)| a > x,
    );

    // println!("depth {:?}", &et.depth);
    // println!("visit {:?}", &et.visit);
    // println!("vcost1 {:?}", &et.vcost1);
    // println!("vcost2 {:?}", &et.vcost2);
    // println!("i {:?}", &et.i);
    // println!("o {:?}", &et.o);

    for (i, &d) in et.depth.iter().take(et.visit.len()).enumerate() {
        // (根からの深さ, 頂点番号)
        seg.set(i, (d, et.visit[i]));
    }
    seg.build();

    // println!("dat {:?}", &seg.dat);

    for (u, v) in qs {
        let mut mi = std::usize::MAX;
        let mut ma = 0;
        for &x in &[et.i[u - 1], et.o[u - 1], et.i[v - 1], et.o[v - 1]] {
            chmin!(mi, x);
            chmax!(ma, x);
        }
        // println!("{:?}", seg.query(mi, ma + 1));
        println!("{:?}", seg.query(mi, ma + 1).1 - 1); // 0-index に戻す
    }
}
