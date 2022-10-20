use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 032 - AtCoder Ekiden（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_af
 *
 * tags: #全探索
 *
 */
#[derive(new)]
struct Rec {
    n: usize,
    a: Vec<Vec<usize>>, // 選手iの区間jを走る時間
    v: Vec<Vec<bool>>,  // 仲悪いリスト
    used: Vec<bool>,
    mi: usize,
}
impl Rec {
    // u: 親, cost: 累計コスト, j: 区間
    fn dfs(&mut self, u: usize, cost: usize, j: usize) {
        if j == self.n {
            // 全選手が走り切った
            self.mi = self.mi.min(cost);
        }
        self.used[u] = true;
        for y in 0..self.n {
            if !self.used[y] && self.v[u][y] {
                self.dfs(y, cost + self.a[y][j], j + 1); // 最後入るとちょうどn区間走ったことになる
                self.used[y] = false;
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize;n];n],
        m: usize,
        r: [(usize, usize); m]
    }

    // 仲が良い=true,仲が悪い=false
    let mut v = vec![vec![true; n]; n];
    for (x, y) in r {
        v[x - 1][y - 1] = false;
        v[y - 1][x - 1] = false;
    }

    let mut mi = std::usize::MAX;
    for u in 0..n {
        let mut rec = Rec::new(n, a.clone(), v.clone(), vec![false; n], std::usize::MAX);
        rec.dfs(u, a[u][0], 1);
        mi = mi.min(rec.mi);
    }

    if mi == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", mi);
    }
}
