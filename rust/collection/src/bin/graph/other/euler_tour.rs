/**
 * @cpg_dirspec euler_tour
 *
 * cpg run -p src/bin/graph/other/euler_tour.rs
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
use library::{graph::euler_tour::*, utils::read::*};

/**
 * オイラーツアー
 *
 * 1. 頂点番号順に 各頂点に初めて訪れた時のタイムスタンプ
 * 2. 頂点番号順に 各頂点を最後に訪れた時のタイムスタンプ
 * 3. 訪れた順に頂点番号を全て管理したリスト（同じ頂点を複数回含む）
 * 4. 各頂点について3.で初めて訪れた位置にコストを配置したリスト
 * 5. 各頂点について3.で初めて訪れた位置にコストを配置、抜けた時にコストの負値を配置したリスト
 * 6. 各頂点について3.で訪れた位置と抜けた時の根からの深さを配置したリスト
 *
 * を半角空白区切りで出力せよ.
 */

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, e) = (a[0], a[1]);
    let mut list = vec![vec![]; n + 1];
    for _ in 0..e {
        let b = read::<usize>();
        let (s, t, w) = (b[0], b[1], b[2] as isize);
        list[s].push(Vertex::new(s, t, w));
        list[t].push(Vertex::new(t, s, w));
    }

    let et = euler_tour(Vertex::new(0, 1, 1), list);

    for x in &et.i {
        print!("{} ", x);
    }
    println!();
    for x in &et.o {
        print!("{} ", x);
    }
    println!();
    for x in &et.visit {
        print!("{} ", x);
    }
    println!();
    for x in &et.vcost1 {
        print!("{} ", x);
    }
    println!();
    for x in &et.vcost2 {
        print!("{} ", x);
    }
    println!();
    for x in &et.depth {
        print!("{} ", x);
    }
    println!();
    // println!("{:?}", &et.p);
    // println!("{:?}", &et.sub);
}
