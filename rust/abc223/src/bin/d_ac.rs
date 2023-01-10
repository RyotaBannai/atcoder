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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

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
 * D - Restricted Permutation
 *
 * https://atcoder.jp/contests/abc223/editorial
 *
 * tags: #topological_sort #トポロジカルソート
 *
 * トポロジカルソートの辺の張り方を逆向きにして、
 * ヒープの二番目の要素を大きい順に後ろから取り出す方法では、WAだった.
 * https://atcoder.jp/contests/abc223/submissions/37923368
 *
 * タスクの前後の依存関係にはトポロジカルソートを使う
 *
 */

fn ab(a: isize) -> usize {
    a.abs() as usize
}

fn neg(a: usize) -> isize {
    -(a as isize)
}

fn main() {
    let a = read::<usize>();
    let (n, m) = (a[0], a[1]);
    let mut list = vec![vec![]; n + 1];
    let mut deg = vec![0; n + 1];
    for _ in 0..m {
        let b = read::<usize>();
        let (s, t) = (b[0], b[1]);
        list[s].push(t); // 有向グラフ
        deg[t] += 1;
    }

    let mut q = BinaryHeap::new();
    for i in 1..=n {
        // 入次数が大きいものが後にくるように負値
        // 頂点番号が大きいものが先にくる(=後ろから見ているから、先頭からみると辞書順で最小)
        q.push((-deg[i], neg(i)));
    }

    let mut ord = vec![];
    let mut used = vec![false; n + 1];
    while let Some((d, u)) = q.pop() {
        if used[ab(u)] {
            continue;
        }
        if d != 0 {
            println!("-1");
            return;
        }
        ord.push(u);
        used[ab(u)] = true;
        for &y in list[ab(u)].iter() {
            if !used[y] {
                deg[y] -= 1;
                q.push((-deg[y], neg(y)));
            }
        }
    }
    for x in ord.iter() {
        print!("{} ", x.abs());
    }
    println!();
}
