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
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 072 - Loop Railway Plan（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_br
 *
 * tags: #全探索 #dfs #バックトラック #条件を満たさないときに一手戻る
 *
 * 考察
 * BitDP でも解ける 典型045
 *
 * 類題：
 * - 第5回アルゴリズム実技検定-G 「ヘビ」
 * - ABC196-D 「Hanjo」
 * - PANASONIC2020-D 「String Equivalence」
 * - ABC119-C 「Synthesic Kadomatsu」
 *
 */

struct Rec {
    h: isize,
    w: isize,
    s: (isize, isize), // 開始地点
    used: Vec<Vec<bool>>,
    ans: usize,
    g: Vec<Vec<char>>,
}
impl Rec {
    fn new(h: usize, w: usize, s: (isize, isize), g: Vec<Vec<char>>) -> Self {
        let used = vec![vec![false; w]; h];
        Self {
            h: h as isize,
            w: w as isize,
            s,
            used,
            ans: 0,
            g,
        }
    }
    fn dfs(&mut self, p: (isize, isize), path_len: usize) {
        if p == self.s && path_len >= 3 {
            self.ans = self.ans.max(path_len);
            return;
        }
        let (y, x) = p;
        for (dx, dy) in vec![(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let (ny, nx) = (y + dy, x + dx);
            if (0 <= ny && ny < self.h) && (0 <= nx && nx < self.w) {
                let (uny, unx) = (ny as usize, nx as usize);
                if !self.used[uny][unx] && self.g[uny][unx] == '.' {
                    self.used[uny][unx] = true;
                    self.dfs((ny, nx), path_len + 1);
                    self.used[uny][unx] = false;
                }
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h]
    }

    let mut ma = 0;
    for i in 0..h {
        for j in 0..w {
            if g[i][j] == '.' {
                let s = (i as isize, j as isize); // row, column
                let mut rec = Rec::new(h, w, s, g.clone());
                rec.dfs(s, 0);
                ma = ma.max(rec.ans);
            }
        }
    }
    if ma == 0 {
        println!("-1");
    } else {
        println!("{}", ma);
    }
}
