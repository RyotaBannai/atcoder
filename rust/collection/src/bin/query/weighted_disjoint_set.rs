/**
 * @cpg_dirspec weighted_disjoint_set
 *
 * cpg run -p src/bin/query/weighted_disjoint_set.rs
 */
// use proconio::{fastout, input, marker::Chars};
use std::io;
use std::usize::MAX;
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
 * D - People on a Line
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/DSL_1_B
 *
 * tags: #weighted_union_find #重み付きunion_find
 *
 *
 * 参考
 * ・https://qiita.com/drken/items/cce6fc5c579051e64fab
 * ・/Users/ryotabannai/Documents/dev/atcoder/atcoder/rust/abc087/src/bin/d_ac.rs
 */

// 0-index
struct WeightedDisjointSet {
    rank: Vec<usize>,
    p: Vec<usize>,
    diff_weight: Vec<isize>,
}
impl WeightedDisjointSet {
    fn new(n: usize) -> Self {
        let mut p = vec![MAX; n];
        let mut rank = vec![0; n];
        for i in 0..n {
            p[i] = i;
            rank[i] = 0;
        }
        Self {
            rank,
            p,
            diff_weight: vec![0; n],
        }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn find(&mut self, x: usize) -> usize {
        if x != self.p[x] {
            let r = self.find(self.p[x]);
            self.diff_weight[x] += self.diff_weight[self.p[x]];
            self.p[x] = r;
        }
        self.p[x]
    }
    fn merge(&mut self, x: usize, y: usize, mut w: isize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        // すでに繋がっている
        if px == py {
            return false;
        }

        let wx = self.weight(x);
        let wy = self.weight(y);

        if self.rank[px] > self.rank[py] {
            w += wx;
            w -= wy;

            self.p[py] = px;
            self.diff_weight[py] = w;
        } else {
            w = -w;
            w -= wx;
            w += wy;

            self.p[px] = py;
            if self.rank[px] == self.rank[py] {
                self.rank[py] += 1;
            }
            self.diff_weight[px] = w;
        }
        true
    }
    fn weight(&mut self, x: usize) -> isize {
        self.find(x); // 経路圧縮
        self.diff_weight[x]
    }
    fn diff(&mut self, x: usize, y: usize) -> isize {
        self.weight(y) - self.weight(x)
    }
}

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, q) = (a[0], a[1]);

    let mut ds = WeightedDisjointSet::new(n);
    for _ in 0..q {
        let b = read::<usize>();
        if b[0] == 0 {
            let (x, y, z) = (b[1], b[2], b[3] as isize);
            ds.merge(x, y, z);
        } else {
            let (x, y) = (b[1], b[2]);
            if ds.same(x, y) {
                println!("{}", ds.diff(x, y));
            } else {
                println!("?");
            }
        }
    }
}
