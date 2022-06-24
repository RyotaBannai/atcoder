use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 *
 * 003 - Longest Circular Road（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_c
 *
 * tags: #グラフ #直径 #dfs #bfs
 *
 * グラフの直径
 *
 */

type Edge = (usize, usize);

#[derive(new)]
struct Rec {
    n: usize,
    d: Vec<usize>,
    g: Vec<Vec<Edge>>,
}

impl Rec {
    // 地点 s からそれぞれの地点への距離を求める
    fn bfs(&mut self, s: usize) {
        self.d = vec![std::usize::MAX; self.n + 1]; // 初期化

        let mut q = VecDeque::new();
        self.d[s] = 0;
        q.push_back(s);
        let mut u;
        while !q.is_empty() {
            u = q.pop_front().unwrap();
            for e in &self.g[u] {
                // まだ通ってない頂点
                if self.d[e.0] == std::usize::MAX {
                    // 親の距離+親から自分の地点までの距離
                    self.d[e.0] = self.d[u] + e.1;
                    q.push_back(e.0);
                }
            }
        }
    }

    // 適当な節点 s から最も遠い節点 x から最も遠い節点 y の距離（dst(x,y)） が木の直径
    // 地点 s からそれぞれの地点への距離を求める
    fn run(&mut self, s: usize) -> usize {
        // s->x
        self.bfs(s);
        let mut ma_v = 0;
        let mut ma_pos = 0;
        for i in 1..=self.n {
            if self.d[i] == std::usize::MAX {
                continue;
            }
            if ma_v < self.d[i] {
                ma_v = self.d[i];
                ma_pos = i;
            }
        }

        // x->y
        // さらに ma_pos から最も遠い接点 ma_v を求める
        self.bfs(ma_pos);
        ma_v = 0;
        for i in 1..=self.n {
            if self.d[i] == std::usize::MAX {
                continue;
            }
            ma_v = max(ma_v, self.d[i]);
        }

        ma_v
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [(usize, usize); n-1]
    }

    let mut g = vec![vec![]; n + 1];
    // 無効グラフ
    for (a, b) in s {
        // 今回はどの頂点間も１
        g[a].push((b, 1));
        g[b].push((a, 1));
    }
    let mut rec = Rec::new(n, vec![], g);

    let ans = rec.run(1);
    // xそれぞれの地点から全探索 これはやらない. 任意の地点から最長距離にある地点からの最長距離に意味があり、初めの地点をどこに選ぶかに意味はない
    // for i in 1..=n {
    //     ans = max(ans, rec.run(i));
    // }

    println!("{}", ans + 1);
}
