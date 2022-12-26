/**
 * @cpg_dirspec tree_euler_tour
 *
 * cpg run -p src/bin/other/aoj/tree.rs
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
use library::{graph::euler_tour::*, query::seg_tree::*, utils::read::*, *};

/**
 * F : Tree / 木
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/2667
 *
 * tags: #EulerTour #オイラーツアー #セグ木 #segment_tree
 *
 * MLE テストケース #27 までok
 *
 * u,v それぞれのpath をオイラーツアーにより求める.
 * 頂点u の根からのパスの距離を d(u) としたとき
 * u,v 間の距離は d(u)+d(v)-2*d(lca)
 *
 * lca を求めるために別のセグ木を用意して、
 * (depth, 頂点番号) を値としたRMQ を求める.
 *
 * 部分木の更新には、in/out それぞれをO(1) で更新. (制約を満たしてないかも)
 * オイラーツアーで求めた vcost2 と同様なin/out の位置を et.i/et.o2 から取得して
 * セグ木のindex を参照してから更新する.
 */

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, q) = (a[0], a[1]);
    let mut list = vec![vec![]; n + 1];
    for i in 1..n {
        // i-index で管理
        let b = read::<usize>();
        let (s, t) = (b[0] + 1, b[1] + 1);
        list[s].push(Vertex::new(s, t, 0)); // 初め重みは全て0
        list[t].push(Vertex::new(t, s, 0));
    }

    // for xs in list.iter() {
    //     println!("{:?}", &xs);
    // }

    let et = euler_tour(Vertex::new(0, 1, 0), list);

    // println!("depth {:?}", &et.depth);
    // println!("visit {:?}", &et.visit);
    // println!("vcost1 {:?}", &et.vcost1);
    // println!("vcost2 {:?}", &et.vcost2);
    // println!("i {:?}", &et.i);
    // println!("o2 {:?}", &et.o2);
    // println!();

    // RSQ_RAQ
    let f = |a: isize, b: isize| a + b;
    let mut seg_dist = LazySegTree::new(
        et.vcost2.len(),
        0,
        0,
        0,
        0,
        f,
        f,
        f,
        |a: isize, n: usize| a * n as isize,
        |a: isize, x: isize| a > x,
    );

    for (i, &d) in et.vcost2.iter().enumerate() {
        seg_dist.set(i, d); // 辺の重み
    }
    seg_dist.build();
    // println!("dat {:?}", &seg.dat);

    // RMQ tuple
    let mut seg_lca = LazySegTree::<(usize, usize)>::new(
        et.visit.len(),
        ((1 << 31) - 1, 0),
        ((1 << 31) - 1, 0),
        ((1 << 31) - 1, 0),
        0,
        |a: (usize, usize), b: (usize, usize)| a.min(b), // min
        |_: (usize, usize), b: (usize, usize)| b,        // update(replace)
        |_: (usize, usize), b: (usize, usize)| b,        // update(replace)
        |a: (usize, usize), _: usize| a,                 // mul 1
        |a: (usize, usize), x: (usize, usize)| a > x,
    );

    for (i, &d) in et.depth.iter().take(et.visit.len()).enumerate() {
        // (根からの深さ, 頂点番号)
        seg_lca.set(i, (d, et.visit[i]));
    }
    seg_lca.build();

    // read query
    for _ in 0..q {
        let b = read::<usize>();
        if b[0] == 0 {
            let (u, v) = (b[1], b[2]);
            let pu = seg_dist.query(0, et.i[u] + 1);
            let pv = seg_dist.query(0, et.i[v] + 1);
            // println!("path:");
            // println!("{}", pu);
            // println!("{}", pv);

            let mut mi = std::usize::MAX;
            let mut ma = 0;
            for &x in &[et.i[u], et.o[u], et.i[v], et.o[v]] {
                chmin!(mi, x);
                chmax!(ma, x);
            }

            let lca = seg_lca.query(mi, ma + 1).1 - 1; // 0-index に戻す
            let plca = seg_dist.query(0, et.i[lca] + 1);

            // println!("lca {}", lca);

            println!("{}", pu + pv - 2 * plca);
        } else {
            //
            let (v, x) = (b[1], b[2] as isize);
            // u は1-index だけど、seg は0-index で管理してる
            // sub の管理では頂点１は0-index に入ってる

            // println!("{:?}", &et.sub[v]);
            for &u in &et.sub[v] {
                let i = et.i[u - 1];
                let o2 = et.o2[u - 1];
                seg_dist.update(i, i + 1, x);
                seg_dist.update(o2, o2 + 1, -x);
            }
            // println!("{:?}", &seg_dist.dat);
        }
    }
}
