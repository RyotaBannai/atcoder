use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet, HashMap};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * E - Crystal Switches
 *
 * https://atcoder.jp/contests/abc277/tasks/abc277_e
 */
use abc277::utils::*;

#[derive(Clone, Copy, Debug, PartialEq, new)]
struct Edge {
    from: usize,
    to: usize,
    cost: usize,
}

#[fastout]
fn main() {
    input! {
        n: usize, // 2<=
        m: usize, // 1<=
        k: usize, // 0<=k
        es: [(usize, usize, usize); m], // 無効グラフ, 通れる・通れないの初期状態
        s: [usize; k], // スイッチがおいてある頂点 1<=
    }

    let mut v = vec![vec![]; n * 2 + 1]; // 連接リスト
    for &e in &es {
        let (from, to, s) = e;
        let (from1, to1) = (from + n, to + n);
        if s == 1 {
            v[from].push(Edge::new(from, to, 1));
            v[to].push(Edge::new(to, from, 1));
        } else {
            v[from1].push(Edge::new(from1, to1, 1));
            v[to1].push(Edge::new(to1, from1, 1));
        }
    }

    for x in s {
        v[x].push(Edge::new(x, x + n, 0));
        v[x + n].push(Edge::new(x + n, x, 0));
    }

    let inf = std::usize::MAX;
    let mut d = vec![inf; n * 2 + 1]; // 距離 スイッチの切り替えの状態（偶奇）で分ける
    d[1] = 0;

    let mut x = BinaryHeap::new();
    x.push_rev((0, 1)); // (d, u)

    while let Some(y) = x.pop_rev() {
        let (ud, u) = y;
        if d[u] < ud {
            continue;
        }
        for &e in &v[u] {
            let nd = ud + e.cost;
            if d[e.to] > nd {
                d[e.to] = nd;
                x.push_rev((nd, e.to));
            }
        }
    }
    // println!("{:?}", &d);

    let mut ans = inf;
    ans = ans.min(d[n]);
    ans = ans.min(d[2 * n]);
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
