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
 * D - Score Attack
 *
 * https://atcoder.jp/contests/abc061/tasks/abc061_d
 *
 * tags: #単一始点最短経路 #SSSP
 *
 * SSSP: 根 s から任意の目的地 t への最短距離
 *
 * ・負の最短距離ではなく問題どおりコストが最大になるよう実装
 * ・コストの符号を反転して最小コストにしても良い
 *
 */

#[derive(Clone, Copy, Debug, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
    cost: isize,
}
impl Edge {
    fn new(from: usize, to: usize, cost: isize) -> Self {
        Self { from, to, cost }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, isize); m]
    }
    // 有向
    let xs = abc
        .iter()
        .map(|&(a, b, w)| Edge::new(a, b, w))
        .collect::<Vec<_>>();

    let inf = std::isize::MIN;
    let mut d = vec![inf; n + 1];
    d[1] = 0; // s == 1

    let mut count = 0;
    while count < n {
        let mut end = true;
        for &x in &xs {
            let nd = d[x.from] + x.cost;
            if d[x.from] != inf && nd > d[x.to] {
                d[x.to] = nd;
                end = false;
            }
        }
        if end {
            break;
        }
        count += 1;
    }

    let ans = d[n];
    // 負の閉路がない
    if count != n {
        println!("{}", ans);
        return;
    }

    // 負の閉路（ゲーム終了時のスコアをいくらでも大きくできる）
    // count == n の場合、経路のどこかで更新が起こるが、
    // 1~n のパスの最中に起こっているとは限らない
    // もう一度 n 頂点分、全辺に対して最短距離の処理を流す.
    // 頂点 n への最短距離が更新されていれば、1~n のパスの最中に負の閉路が発生していると考えられる

    // グラフだから、上記のが起きることは考えられない？

    // 1~n のステップで、一つずつ頂点を進むようなグラフが構成されていて、
    // n からの頂点が 1~n の頂点に辺を順に伸ばしている場合に、
    // 今回の 1~n の走査をした結果、負の閉路が見つからなくても、
    // count == n になってしまう.
    // そのため、再チェックし、最短距離が更新されなければ、負の閉路なし、と判定する
    for _ in 0..n {
        for &x in &xs {
            let nd = d[x.from] + x.cost;
            if d[x.from] != inf && nd > d[x.to] {
                d[x.to] = nd;
            }
        }
    }

    if ans != d[n] {
        println!("inf");
    } else {
        println!("{}", ans);
    }
}
