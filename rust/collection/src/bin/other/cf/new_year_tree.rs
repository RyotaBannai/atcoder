/**
 * @cpg_dirspec new_year_tree
 *
 * cpg run -p src/bin/other/cf/new_year_tree.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, HashSet};
// type Map = BTreeMap<String, usize>;
// type Set = HashSet<isize>;
// use std::collections::{BinaryHeap, VecDeque};
use library::{
    graph::{euler_tour::*, vertex::*},
    query::seg_tree::*,
    scan,
    utils::{scanner::Scanner, writer::Writer},
};

/**
 * E. New Year Tree
 *
 * https://codeforces.com/contest/620/problem/E
 *
 * tags: #EulerTour #オイラーツアー #セグ木 #segment_tree
 *
 */

// #[fastout]
fn main() {
    let mut sc = Scanner::stdin();
    let mut wr = Writer::stdout();
    let (n, q) = scan!(sc, usize, usize);
    let c = scan!(sc, i8; n); // color
    let mut list = vec![vec![]; n + 1];
    for _ in 1..n {
        // i-index で管理
        let (s, t) = scan!(sc, usize, usize);
        list[s].push(Vertex::new(s, t, 0)); // 初め重みは全て0
        list[t].push(Vertex::new(t, s, 0));
    }

    let et = euler_tour(Vertex::new(0, 1, 0), &list);

    // RSQ_RAQ
    let mut seg = LazySegTree::new(
        et.vcost2.len(),
        0u64,
        0u64,
        0,
        0,
        |a: u64, b: u64| a | b, // or
        |_: u64, _: isize, _: usize| unimplemented!(),
        |_: u64, x: isize, _: usize| 1 << (x - 1), // この区間は全部x
        |_: isize, _: isize| unimplemented!(),
        |_: isize, b: isize| b,
        |_: u64, _: u64| unimplemented!(),
    );

    for (i, u) in et.visit.iter().enumerate() {
        seg.set(i, 1 << (c[u - 1] - 1)); // 色を初期値にする
    }
    seg.build();

    // read query
    for _ in 0..q {
        let m = scan!(sc, usize);
        if m == 2 {
            let v = scan!(sc, usize) - 1;
            let i = et.i[v];
            let o2 = et.o2[v];
            let num = seg.query(i, o2);
            wr.write(num.count_ones()).unwrap();
        } else {
            let (v, x) = (scan!(sc, usize) - 1, scan!(sc, isize));
            let i = et.i[v];
            let o2 = et.o2[v];
            seg.update(i, o2 + 1, x);

            // 意図的にセグ木のlazy を更新.
            // seg.force_update();
            // 葉の状態を表示.
            // seg.show_leafs();
            // println!();
        }
    }
}
