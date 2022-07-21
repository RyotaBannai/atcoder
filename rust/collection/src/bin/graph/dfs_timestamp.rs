/**
 * @cpg_dirspec dfs_timestamp
 *
 * cpg run -p src/bin/graph/dfs_timestamp.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
use std::io;
use std::usize::MAX;
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * DFS 行きがけ、帰りがけ（発見時刻、完了時刻）
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/ALDS1_11_B
 *
 * tags: #DFS #行きがけ #帰りがけ
 *
 */

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

struct Rec {
    // parent: Vec<usize>,
    pre: usize,
    ord: Vec<usize>,
    ord_rev: Vec<usize>,
    list: Vec<Vec<usize>>, // グラフの連接リスト
}
impl Rec {
    fn new(list: Vec<Vec<usize>>) -> Self {
        let n = list.len();
        Self {
            pre: 1,
            ord: vec![MAX; n],
            ord_rev: vec![MAX; n],
            list,
        }
    }

    fn dfs(&mut self, u: usize) {
        self.ord[u] = self.pre;
        self.pre += 1;
        // 葉だった場合、自分より先が無いため、for-in をパスして、
        // そのまま帰りがけの timestamp を入れる
        for &v in &self.list[u].clone() {
            if self.ord[v] == MAX {
                // 未訪問
                self.dfs(v);
            }
        }

        // 親に戻って、次の子を探索する際は、初めの子の帰りがけの increment を行きがけの timestamp として使う

        self.ord_rev[u] = self.pre;
        self.pre += 1;
    }
}

// #[fastout]
fn main() {
    // aoj inp
    let a = read::<usize>();
    let n = a[0];
    let mut t = vec![vec![]; n + 1];
    for i in 1..=n {
        let b = read::<usize>();
        t[i] = b[2..2 + b[1]].to_vec();
    }

    // println!("{:?}", t);
    let mut rec = Rec::new(t);

    // 根 s==1
    // グラフが連結とは限らない場合なので、
    // 全頂点から探索する
    // 非連結の場合は、s==1 を含むグラフ(T)を探索してから、
    // 別のグラフを T の探索完了時点での timestamp から再探索する
    for i in 1..=n {
        if rec.ord[i] == MAX {
            // 未訪問
            rec.dfs(i);
        }
    }

    for (i, (ord, ord_rev)) in rec.ord.iter().zip(rec.ord_rev).enumerate().skip(1) {
        println!("{} {} {}", i, ord, ord_rev);
    }
}
