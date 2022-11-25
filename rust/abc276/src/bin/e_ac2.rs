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
// use abc276::utils::*;

/**
 * E - Round Trip
 *
 * https://atcoder.jp/contests/abc276/tasks/abc276_e
 *
 * tags: #bfs
 *
 * 考察
 * 盤の大きさは高々 10^6 だから全探索しても間に合う.
 * またSから出て、Sに戻るというのは、S の隣接する頂点のうち、２つを選んでそのうち一つから出発して、もう一つの方に到着する、というように考えられる
 * 合計の計算量は 4C2*10^6
 *
 * 各隣接する頂点から出発して、同じ隣接する頂点から辿る道を同じグループとして管理する. 他のグループで塗られた道に到達した時に、その距離が4以上ならYes
 *
 * 問題サンプル１を処理した時に埋まるマスの例
 * マスの値は（グループ、頂点S に隣接する辺からの距離+1）
 *
 * [(1, 3), (1, 2), (1, 3), (1, 4)]
 * [(-1, -1), (1, 1), (-1, -1), (2, 3)]
 * [(0, 1), (-1, -1), (2, 1), (2, 2)]
 * [(0, 2), (-1, -1), (-1, -1), (2, 3)]
 *
 * マス取りゲーム　みたいなイメージ.
 *
 *
 *  BFS でもAC.
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h]
    }

    let mut p = (0, 0);
    let neis = [(0, -1isize), (-1isize, 0), (0, 1), (1, 0)];
    let mut pos = vec![];
    for i in 0..h {
        for j in 0..w {
            if g[i][j] == 'S' {
                p = (i, j);
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

    // 2d の盤を1d で表現するために計算. h=5, w=3, 0index, (i,j)=(1,2) 1*3 + 2 =5
    // let calc = |i: isize, j: isize| (i * w as isize + j) as usize;
    // let mut ds = WeightedDisjointSet::new(h * w); // 0 indexed

    let mut used = vec![vec![(-1isize, -1isize); w]; h];
    let mut q = VecDeque::new();
    for (i, &x) in pos.iter().enumerate() {
        q.push_back((x, i as isize, 1)); // parent, group, distance

        // used[x.0 as usize][x.1 as usize] = (i as isize, 1); // group, distance // ここでやらない
    }

    while let Some(((i, j), marker, d)) = q.pop_front() {
        let (group, dist) = used[i as usize][j as usize];
        if group != -1 {
            // すでにどのグループかが通った.
            if group != marker && d + dist >= 3 {
                println!("Yes");
                return;
            } else {
                // 通ったのが同じグループか、距離が 3 以下の場合
                continue;
            }
        } else {
            // まだどのグループも通っていないから、今回のグループで塗る
            used[i as usize][j as usize] = (marker, d);
        }

        for (di, dj) in &neis {
            let (ni, nj) = (i as isize + di, j as isize + dj);
            if (ni < 0 || ni >= h as isize)
                || (nj < 0 || nj >= w as isize)
                || g[ni as usize][nj as usize] == '#'
                || g[ni as usize][nj as usize] == 'S'
            {
                continue;
            }
            q.push_back(((ni, nj), marker, d + 1))
        }
    }
    // for xs in used {
    //     println!("{:?}", &xs);
    // }

    println!("No");
}
