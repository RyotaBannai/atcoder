use itertools::iproduct;
use library::chmax;
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
use std::collections::{BinaryHeap, VecDeque};

/**
 * Maze Master
 *
 * https://atcoder.jp/contests/abc151/tasks/abc151_d
 *
 * tags: #DFS
 *
 */

// #[fastout]
fn main() {
    input! {
        h: isize,
        w: isize,
        g: [Chars; h]
    }

    let mut ans = 0;
    for (i, j) in iproduct!(0..h, 0..w) {
        // 壁から開始しないように注意.
        if g[i as usize][j as usize] == '#' {
            continue;
        }
        let mut dist = vec![vec![-1; w as usize]; h as usize];
        // 開始地点を初期化
        dist[i as usize][j as usize] = 0;
        let mut q = VecDeque::new();
        q.push_back((i, j, 0)); // r,c
        while let Some((r, c, d)) = q.pop_front() {
            let moves = vec![(-1isize, 0), (0, -1isize), (1, 0), (0, 1)];
            for (dr, dc) in moves {
                let (nr, nc) = (r + dr, c + dc);
                if nr < 0 || h <= nr || nc < 0 || w <= nc {
                    continue;
                }
                if g[nr as usize][nc as usize] == '#' {
                    continue;
                }
                let rf = &mut dist[nr as usize][nc as usize];
                if *rf != -1 {
                    continue;
                }
                *rf = d + 1;
                q.push_back((nr, nc, d + 1));
            }
        }
        chmax!(ans, dist.into_iter().flatten().max().unwrap());
    }
    println!("{}", ans);
}
