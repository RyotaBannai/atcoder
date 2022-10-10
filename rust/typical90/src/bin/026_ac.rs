use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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

/**
 * 026 - Independent Set on a Tree（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_z
 *
 * tags: #二部グラフ
 *
 * それぞれの頂点が隣接しない組を２つ作る(二部グラフ)
 * 各組の大きさを調べる.
 * 大きい方から'だけ'選べば N/2 以上選べる
 *
*/

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1]
    }

    let mut t = vec![vec![]; n];
    for (a, b) in ab {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut q = VecDeque::new();
    let mut used = vec![false; n];
    q.push_back((0, 0)); // 偶奇, 頂点番号
    used[0] = true;
    let mut vs = vec![vec![]; 2];

    while let Some((tp, u)) = q.pop_back() {
        vs[tp].push(u);
        for &y in &t[u] {
            if !used[y] {
                q.push_back(((tp + 1) % 2, y));
                used[y] = true;
            }
        }
    }

    let i = if vs[0].len() > vs[1].len() { 0 } else { 1 }; // 大き方
    for x in vs[i].iter().take((n + 1) / 2) {
        print!("{} ", x + 1);
    }
}
