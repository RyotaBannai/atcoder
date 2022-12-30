/**
 * @cpg_dirspec tree_euler_tour
 *
 * cpg run -p src/bin/other/aoj/tree_euler_tour.rs
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
 * F : Tree / 木
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/2667
 *
 * tags: #EulerTour #オイラーツアー #セグ木 #segment_tree
 *
 * u,v それぞれのpath をオイラーツアーにより求める.
 * 頂点u の根からのパスの距離を d(u) としたとき
 * u,v 間の距離は d(u)+d(v)-2*d(lca)
 *
 * lca を求めるために別のセグ木を用意して、
 * (depth, 頂点番号) を値としたRMQ を求める.
 *
 * 部分木の更新には、部分木の数k とした時O(k) で更新.
 * 親のin/out の区間では更新できない. 子が３つ以上になるときに、両端以外の頂点に入る時に一度親に戻るvisit があるため、この子への辺に親の重さw が入ってしまう.
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
        list[s].push(Vertex::new(s, t, 1)); // 初め重みは全て0

        // list[t].push(Vertex::new(t, s, 1));
    }

    // for xs in list.iter() {
    //     println!("{:?}", &xs);
    // }

    let et = euler_tour(Vertex::new(0, 1, 1), &list);

    // println!("depth {:?}", &et.depth);
    // println!("visit {:?}", &et.visit);
    // println!("vcost1 {:?}", &et.vcost1);
    // println!("vcost2 {:?}", &et.vcost2);
    // println!("i {:?}", &et.i);
    // println!("o {:?}", &et.o);
    // println!("o2 {:?}", &et.o2);
    // println!();

    type Det = (isize, usize, usize);

    // RSQ_RAQ
    let f = |a: isize, b: isize| a + b;
    let mut seg_dist = LazySegTree::new(
        et.vcost2.len(),
        (0, 0, 0),
        (0, 0, 0),
        0,
        0,
        |a: Det, b: Det| (a.0 + b.0, a.1 + b.1, a.2 + b.2),
        |a: Det, x: isize| (a.0 + x * (a.1 as isize - a.2 as isize), a.1, a.2),
        f,
        |a: isize, _: usize| a,
        |a: Det, x: Det| a.0 > x.0,
    );

    for (i, &d) in et.vcost2.iter().enumerate() {
        // 葉を初期化.
        // 今回は初期値0.
        // 2,3 はオイラーツアーおける正負
        // (0,正の位置なら1, 負の位置なら1)
        seg_dist.set(i, (0, if d > 0 { 1 } else { 0 }, if d > 0 { 0 } else { 1 }));
    }
    seg_dist.build();

    // RMQ tuple
    let mut seg_lca = LazySegTree::new(
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

    // println!();

    // read query
    for _ in 0..q {
        let b = read::<usize>();
        if b[0] == 0 {
            let (u, v) = (b[1], b[2]);
            let pu = seg_dist.query(0, et.i[u] + 1);
            let pv = seg_dist.query(0, et.i[v] + 1);

            let mut mi = std::usize::MAX;
            let mut ma = 0;
            for &x in &[et.i[u], et.o2[u], et.i[v], et.o2[v]] {
                chmin!(mi, x);
                chmax!(ma, x);
            }
            // max+1 としない.. depth は負の位置はすでに上の階層に戻っているから、上の階層+1 にすると右側の開区間がちょうど一つ浅いdepth を含んでしまう.
            // または o を使ったときの max+1 はok
            let lca = seg_lca.query(mi, ma).1 - 1; // 0-index に戻す
            let plca = seg_dist.query(0, et.i[lca] + 1);

            // println!("visit {:?}", &et.visit);
            // println!("depth {:?}", &et.depth);
            // println!("path:");
            // println!("u,v {},{}", u + 1, v + 1);
            // println!("{:?}", pu);
            // println!("{:?}", pv);
            // println!("{:?}", plca);
            // println!("lca {}", lca);

            println!("{}", pu.0 + pv.0 - 2 * plca.0);
        } else {
            //
            let (v, x) = (b[1], b[2] as isize);
            // u は1-index だけど、seg は0-index で管理してる
            // sub の管理では頂点１は0-index に入ってる

            // それぞれの部分木だけに更新をかける.(部分木への辺に更新更新をかける.)
            for u in &list[v + 1] {
                let i = et.i[u.to - 1];
                let o2 = et.o2[u.to - 1];
                seg_dist.update(i, o2 + 1, x);
            }

            // 意図的にセグ木のlazy を更新.
            // seg_dist.force_update();
            // 葉の状態を表示.
            // seg_dist.show_leafs();
            // println!();
        }
    }
}
