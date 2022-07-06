/**
 * @cpg_dirspec
 *
 * cpg run -p src/bin/graph/bipartite_graph.rs
 */
use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
use std::io;
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * 二部マッチング
 *
 * https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_7_A&lang=jp
 *
 * tags: #部マッチング #マッチング #二部グラフ
 *
 * https://qiita.com/drken/items/a803d4fc4a727e02f7ba#4-3-%E4%BA%8C%E9%83%A8%E3%82%B0%E3%83%A9%E3%83%95%E5%88%A4%E5%AE%9A
 */

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

/**
 * 二部グラフ判定
 *
 * すでに通った隣会う頂点の距離の mod 2 が同じ = 同じカテゴリに属す頂点どうしで false
 * または色(0/1)で判定. 親の色と別の色で子供を塗る. !-1 の時に、同じ色かどうかを判定. 同じ色で false
 *
 */

#[fastout]
fn main() {
    // aoj inp
    let a = read::<usize>();
    let (n, e) = (a[0], a[1]); // n 頂点数、e 辺数
    let mut v = vec![vec![]; n];
    for _ in 0..e {
        let b = read::<usize>();
        let (s, t) = (b[0], b[1]);
        v[s].push(t); // 無向
        v[t].push(s);
    }

    let mut dist = vec![-1; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    dist[0] = 0; // グラフが連結しているかわからない場合は各頂点から探索. vec を持って探索済みかどうか管理して、未探索頂点なら処理する

    let mut ok = true;
    while !q.is_empty() {
        let u = q.pop_back().unwrap();
        for &w in &v[u] {
            if dist[w] != -1 {
                // すでに通った
                ok &= dist[w] % 2 != dist[u] % 2;
            } else {
                dist[w] = dist[u] + 1;
                q.push_back(w);
            }
        }
    }

    print!("{}", if ok { "Yes" } else { "No" });
}
