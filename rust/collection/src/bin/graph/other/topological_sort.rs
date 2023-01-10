/**
 * @cpg_dirspec topological_sort
 *
 * cpg run -p src/bin/graph/other/topological_sort.rs
 */
use std::io;
use std::usize::MAX;
// use proconio::{fastout, input, marker::Chars};
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
use std::collections::{BinaryHeap, VecDeque};

use library::utils::read::*;

/*
トポロジカルソート 依存関係を解決する処理

無向グラフではサイクルのないグラフは森 (木を集めたもの)
有向グラフではサイクルのないグラフは DAG (Directed Acyclic Graph)

出次数 deg[v] 頂点 v を始点とする辺の個数
シンク　出次数が 0 であるような頂点（つまり他のタスクに依存していない頂点）

シンクに向かって伸びている辺たちを、元のグラフから削除することを繰り返す
*/

/**
 * トポロジカルソート
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/GRL_4_B
 *
 * tags: #topological_sort #トポロジカルソート
 *
 */

fn main() {
    let a = read::<usize>();
    let (v, e) = (a[0], a[1]);
    let mut list = vec![vec![]; v + 1];
    let mut deg = vec![0; v + 1];
    for _ in 0..e {
        let b = read::<usize>();
        let (s, t) = (b[0], b[1]);
        list[t].push(s); // 有向グラフ // 逆向きに張る
        deg[s] += 1; // 依存されてた側が依存される側になる
    }

    let mut q = BinaryHeap::new();
    for i in 0..v {
        // 入次数が大きいものが後にくるように負値
        // 頂点番号が大きいものが先にくる(=後ろから見ているから、先頭からみると辞書順で最小)
        q.push((-deg[i], i));
    }

    let mut ord = vec![];
    let mut used = vec![false; v + 1];
    while let Some((d, u)) = q.pop() {
        if used[u] {
            continue;
        }
        if d != 0 {
            println!("-1");
            return;
        }
        ord.push(u);
        used[u] = true;
        for &y in list[u].iter() {
            if !used[y] {
                deg[y] -= 1;
                q.push((-deg[y], y));
            }
        }
    }
    for x in ord.iter().rev() {
        print!("{} ", x);
    }
    println!();
}
