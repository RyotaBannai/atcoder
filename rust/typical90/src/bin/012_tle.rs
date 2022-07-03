use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

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
 * ２は？最大 2000x2000 だから大丈夫そう..
 *      どの方向に向かうかがポイント
 *      r2,c2 を goal として迷路の全探索？　枠を決めるのがきも？
 *
 * ・used で一度通った箇所 true
 * ・priority queue で距離が短い順
 *
 * TLE
 */

#[ext]
impl<T: Ord> BinaryHeap<Reverse<T>> {
    fn peek_rev(&self) -> Option<&T> {
        self.peek().map(|Reverse(v)| v)
    }
    fn push_rev(&mut self, x: T) {
        self.push(Reverse(x))
    }
    fn pop_rev(&mut self) -> Option<T> {
        self.pop().map(|Reverse(u)| u)
    }
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    // 0: 白、1: 赤
    let mut t = vec![vec![0; w + 1]; h + 1];
    for _ in 0..q {
        input! { a: usize }
        if a == 1 {
            input! {
                r: usize,
                c: usize,
            }
            t[r][c] = 1;
        } else {
            input! {
                r1: usize,
                c1: usize,
                r2: usize,
                c2: usize,
            }
            if t[r1][c1] == 0 || t[r2][c2] == 0 {
                println!("No");
                continue;
            }
            let moves: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]; // 上下左右 斜め移動はない

            // let mut qu = VecDeque::new();
            let mut qu: BinaryHeap<Reverse<(usize, usize, usize)>> = BinaryHeap::new();
            qu.push_rev((0, r1, c1));

            let mut used = vec![vec![false; w + 1]; h + 1];
            used[r1][c1] = true;

            let mut ok = false;
            while !qu.is_empty() {
                let u = qu.pop_rev().unwrap();
                if u.1 == r2 && u.2 == c2 {
                    ok = true;
                    break;
                }

                for (dr, dc) in &moves {
                    let nr = (u.1 as isize + dr) as usize;
                    let nc = (u.2 as isize + dc) as usize;

                    let r_pred = nr >= 1 && nr <= h;
                    let c_pred = nc >= 1 && nc <= w;
                    if r_pred && c_pred && t[nr][nc] == 1 && !used[nr][nc] {
                        let prio =
                            (nr as isize - r2 as isize).abs() + (nc as isize - c2 as isize).abs();
                        qu.push_rev((prio as usize, nr, nc));
                        used[nr][nc] = true;
                    }
                }
            }

            println!("{}", if ok { "Yes" } else { "No" });
        }
    }
}
