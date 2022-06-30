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
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

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
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    // 0: 白、1: 赤
    let mut t = vec![vec![0; w + 1]; h + 1];
    for i in 0..q {
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
            // let r_mi = min(r1, r2);
            // let r_ma = max(r1, r2);
            // let c_mi = min(c1, c2);
            // let c_ma = max(c1, c2);
            let moves: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]; // 上下左右 斜め移動はない
            let mut qu = VecDeque::new();
            qu.push_back((r1, c1));

            let mut ok = false;
            while !qu.is_empty() {
                let u = qu.pop_back().unwrap(); // dfs
                if u.0 == r2 && u.1 == c2 {
                    ok = true;
                    break;
                }

                for (dr, dc) in &moves {
                    let nr = (u.0 as isize + dr) as usize;
                    let nc = (u.1 as isize + dc) as usize;

                    // let r_pred = nr >= r_mi && nr <= r_ma;
                    // let c_pred = nc >= c_mi && nc <= c_ma;
                    let r_pred = nr >= 1 && nr <= h;
                    let c_pred = nc >= 1 && nc <= w;
                    if r_pred && c_pred && t[nr][nc] == 1 {
                        // 範囲内で
                        qu.push_back((nr, nc));
                    }
                }
            }

            println!("{}", if ok { "Yes" } else { "No" });
        }
    }
}
