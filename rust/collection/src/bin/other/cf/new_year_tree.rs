// use itertools::Itertools;
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
use std::collections::{BTreeMap, HashSet};
// type Map = BTreeMap<String, usize>;
type Set = HashSet<isize>;
// use std::collections::{BinaryHeap, VecDeque};
use library::{
    graph::{euler_tour::*, vertex::*},
    query::seg_tree::*,
    utils::read::*,
    *,
};

/**
 * E. New Year Tree
 *
 * https://codeforces.com/contest/620/problem/E
 *
 * tags: #EulerTour #オイラーツアー #セグ木 #segment_tree
 *
 * MLE テストケース #50 まではpass
 *
 */

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, q) = (a[0], a[1]);
    let c = read::<usize>(); // color
    let mut list = vec![vec![]; n + 1];
    for i in 1..n {
        // i-index で管理
        let b = read::<usize>();
        let (s, t) = (b[0], b[1]);
        list[s].push(Vertex::new(s, t, 0)); // 初め重みは全て0
        list[t].push(Vertex::new(t, s, 0));
    }

    // for xs in list.iter() {
    //     println!("{:?}", &xs);
    // }

    let et = euler_tour(Vertex::new(0, 1, 0), list.clone());

    // println!("depth {:?}", &et.depth);
    // println!("visit {:?}", &et.visit);
    // println!("vcost1 {:?}", &et.vcost1);
    // println!("vcost2 {:?}", &et.vcost2);
    // println!("i {:?}", &et.i);
    // println!("o {:?}", &et.o);
    // println!("o2 {:?}", &et.o2);
    // println!();

    // RSQ_RAQ
    let mut seg = LazySegTree::new(
        et.vcost2.len(),
        Set::new(),
        Set::new(),
        0,
        0,
        |mut a: Set, mut b: Set| a.union(&mut b).cloned().collect::<Set>(),
        // この区間は全部x
        |s: Set, x: isize| {
            let mut ret = Set::new();
            ret.insert(x);
            ret
        },
        |_: isize, b: isize| b, // update(replace)
        |a: isize, _: usize| a,
        |a: Set, x: Set| unimplemented!(),
    );

    for (i, u) in et.visit.iter().enumerate() {
        let mut s = Set::new();
        s.insert(c[u - 1] as isize); // 色を初期値にする
        seg.set(i, s);
    }
    seg.build();

    // println!("{:?}", &seg.dat);
    // println!();

    // read query
    for _ in 0..q {
        let b = read::<usize>();
        if b[0] == 2 {
            let v = b[1] - 1;
            let i = et.i[v];
            let o2 = et.o2[v];
            let or = seg.query(i, o2);
            println!("{}", or.len());
        } else {
            let (v, x) = (b[1] - 1, b[2] as isize);

            // 頂点ごと部分木に更新をかける.
            let i = et.i[v];
            let o2 = et.o2[v];
            seg.update(i, o2 + 1, x);

            // 意図的にセグ木のlazy を更新.
            // println!("{:?}", v + 1);
            // println!("{:?}", &list[v + 1]);
            // seg.force_update();

            // 葉の状態を表示.
            // println!("{:?}", &seg.dat);
            // let nn = seg.n;
            // println!("visit {:?}", &et.visit);
            // seg.show_leafs();
            // println!();
        }
    }
}

/*
10 10
23 25 23 42 23 53 49 40 28 44
1 7
1 2
2 4
4 10
8 10
6 8
3 8
5 3
9 5
2 10
1 6 52
1 8 43
2 3
1 4 39
1 8 44
1 9 39
2 1
2 4
1 6 36

*/
