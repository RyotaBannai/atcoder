use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
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
 * 012 - Red Painting（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_l
 *
 * H 行 W 列のマス目があり、上から i (1≤i≤H) 行目、左から j (1≤j≤W) 列目のマスを (i,j) と表します。
 *
 * Q 個のクエリが与えられる
 * 1 なら、r,c マスを赤く塗る
 * 2 なら、以下の条件をどちらも満たす時に　Yes そうでない時 No と出力する
 *  ・r1,c1 マスと r2,c2 マスがどちらも赤色
 *  ・r1,c1 マスから r2,c2 マスまで赤いろのマス上を上下左右移動することで辿り着ける
 *
 *
 * １は時間かからない
 * ２は？最大 2000x2000 で全探索すると間に合わない
 *    連結判定が重い
 *    連結判定には Union-Find
 *
 * 要素同士の親が同じかどうかを判定
 *
 * 各要素は r * w + c でユニークな数値を与える
 *
 * disjoint set の parent とは別に、赤で塗られたものを管理したい
 * ・ds の parent はデフォルトで赤白関係なく用意しているから、赤の判定はできない
 * → デフォルトで全て用意せずに、赤で塗る時に parent 追加できるようにする
 * 赤で塗るときに、上下左右が赤なら全て連結したい
 *
 */

struct DisjointSet {
    rank: Vec<usize>,
    x: Vec<bool>,
    p: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut p = vec![1; n + 1];
        let mut rank = vec![1; n + 1];
        for i in 1..=n {
            p[i] = i;
            rank[i] = 0;
        }
        let x = vec![false; n + 1];
        Self { rank, x, p }
    }
    fn add(&mut self, i: usize) {
        self.x[i] = true;
    }
    fn has(&mut self, i: usize) -> bool {
        self.x[i]
    }
    // x に入っているかどうか判定しない
    fn find(&mut self, i: usize) -> usize {
        if i != self.p[i] {
            self.p[i] = self.find(self.p[i]);
        }
        self.p[i]
    }
    fn same(&mut self, i: usize, j: usize) -> bool {
        if !self.has(i) || !self.has(j) {
            return false;
        }
        self.find(i) == self.find(j)
    }
    fn unite(&mut self, i: usize, j: usize) {
        if !self.has(i) || !self.has(j) {
            return;
        }
        let p1 = self.find(i);
        let p2 = self.find(j);
        self.link(p1, p2);
    }
    // x に入っているかどうか判定しない
    fn link(&mut self, i: usize, j: usize) {
        if self.rank[i] > self.rank[j] {
            self.p[j] = i; // ランクが大き方につける
        } else {
            self.p[i] = j;
            if self.rank[i] == self.rank[j] {
                // rank 更新前は同じ可能性がある
                self.rank[j] += 1;
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut ds = DisjointSet::new(h * w);
    let mut pos = vec![vec![0; w + 1]; h + 1];
    let moves = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]; //  上下左右
    for i in 1..=h {
        for j in 1..=w {
            pos[i][j] = (i - 1) * w + j;
        }
    }

    for _ in 0..q {
        input! {
          t: usize
        }
        if t == 1 {
            input! {
                r: usize, c: usize,
            }
            let p = pos[r][c];
            ds.add(p); // 赤に塗る
            for (dr, dc) in &moves {
                // 上下左右を繋げるかどうか確認
                let nr = ((r as isize) + dr) as usize;
                let nc = ((c as isize) + dc) as usize;
                let r_pred = nr >= 1 && nr <= h;
                let c_pred = nc >= 1 && nc <= w;
                if !r_pred || !c_pred {
                    // マスが範囲外
                    continue;
                }
                let np = pos[nr][nc];
                ds.unite(p, np); // 隣も赤なら繋げる
            }
        } else {
            input! {
                r1: usize, c1: usize,
                r2: usize, c2: usize,
            }
            let p1 = pos[r1][c1];
            let p2 = pos[r2][c2];
            let ok = ds.same(p1, p2); // 連結しているかどうか確認

            println!("{}", if ok { "Yes" } else { "No" });
        }
    }
}
