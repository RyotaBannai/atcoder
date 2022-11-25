use std::collections::VecDeque;

use proconio::{fastout, input, marker::Chars};
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
use abc276::graph_lib::max_flow::*;

/**
 * E - Round Trip
 *
 * https://atcoder.jp/contests/abc276/tasks/abc276_e
 *
 * tags: #最大流 #max_flow
 *
 * マスS から最大流量２流れる= マスS に隣接するマスが二つ以上あり、一方の流量１がもう一方のマスに流れ込む（もう一つの流量１はマスSから直接流れ込む）、ということ
 * = マスSに隣接する２つマスが連結している
 *
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h]
    }
    // 2d の盤を1d で表現するために計算. h=5, w=3, 0index, (i,j)=(1,2) 1*3 + 2 =5
    let calc = |i: isize, j: isize| (i * w as isize + j) as usize;
    let mut graph = Graph::new(h * w + 1);

    for i in 0..h {
        for j in 0..w {
            if g[i][j] == '#' {
                continue;
            }
            let pos = calc(i as isize, j as isize);
            // 全マスにおいて、右下だけ見れば全てのマスを重複なく連結できる
            for (di, dj) in &[(0, 1), (1, 0)] {
                let (ni, nj) = (i as isize + di, j as isize + dj);
                if (ni < 0 || ni >= h as isize)
                    || (nj < 0 || nj >= w as isize)
                    || g[ni as usize][nj as usize] == '#'
                {
                    continue;
                }
                let nei_pos = calc(ni, nj);
                // 距離はマス一つ分だから costは １で良い
                // 無効グラフだから両方向から貼る（蟻本p192）
                graph.add_edge(pos, nei_pos, 1);
                graph.add_edge(nei_pos, pos, 1);
            }
        }
    }

    let mut s = 0;
    // S の隣接するマスを探す.
    let neis = [(0, -1isize), (-1isize, 0), (0, 1), (1, 0)];
    let mut pos = vec![];
    for i in 0..h {
        for j in 0..w {
            if g[i][j] == 'S' {
                s = calc(i as isize, j as isize);
                for (di, dj) in &neis {
                    let (ni, nj) = (i as isize + di, j as isize + dj);
                    if ni < 0
                        || ni >= h as isize
                        || nj < 0
                        || nj >= w as isize
                        || g[ni as usize][nj as usize] == '#'
                    {
                        continue;
                    }
                    pos.push((ni, nj));
                }
                break;
            }
        }
    }
    for x in pos {
        let t = calc(x.0, x.1);
        let mut ford = FordFulkerson::new(graph.clone());
        let max_dist = ford.max_flow(s, t);
        if max_dist >= 2 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
