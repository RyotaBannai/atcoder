use proconio::{fastout, input, marker::Chars};
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
 * D - Built?
 *
 * https://atcoder.jp/contests/abc065/tasks/arc076_b
 *
 * 最小全域木で解く
 * クラスカル法
 * ・全辺を昇順ソートする
 * ・ソート順に最小全域木 T にその頂点が含まれていない場合は T に新たに追加する
 * ・含まれているかどうかの判定には、UninonFind を用いると高速
 */
use abc065::utils::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        e: [(isize, isize); n]
    }
    let mut xs = vec![];

    // それぞれの島は 0-index で考える
    let mut m = e
        .into_iter()
        .enumerate()
        .map(|(i, (x, y))| (i, x, y))
        .collect::<Vec<(usize, isize, isize)>>();

    // 事前にソートしておくと、隣合わせの距離を比較するだけで良い
    // 組み合わせ(i:0~n,j:i+1~n) 𝑂(𝑉^2) 、ソート 𝑂(|𝐸|log|𝑉|)
    m.sort_unstable_by(|(_, a, _), (_, b, _)| a.cmp(b)); // xでソート
    for i in 0..n - 1 {
        let (u, x1, _) = m[i];
        let (v, x2, _) = m[i + 1];
        xs.push(((x2 - x1).abs(), u, v)); // 有効だけでok
    }

    m.sort_unstable_by(|(.., a), (.., b)| a.cmp(b)); // yでソート
    for i in 0..n - 1 {
        let (u, _, y1) = m[i];
        let (v, _, y2) = m[i + 1];
        xs.push(((y2 - y1).abs(), u, v)); // 有効だけでok
    }

    xs.sort_unstable_by(|(a, ..), (b, ..)| a.cmp(b)); // 重みでソート

    let mut sum = 0usize;
    let mut ds = DisjointSet::new(n);
    for x in xs {
        let (w, i, j) = x;
        if !ds.same(i, j) {
            ds.unite(i, j);
            sum += w as usize;
        }
    }

    println!("{}", sum);
}
