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
 * 考察
 * 盤の大きさは高々 10^6 だから全探索しても間に合う.
 * またSから出て、Sに戻るというのは、S の隣接する頂点のうち、２つを選んでそのうち一つから出発して、もう一つの方に到着する、というように考えられる
 * 合計の計算量は 4C2*10^6
 *
 * 一つの隣接頂点から出発して、もう一つの隣接頂点に到着できる全探索して、そのような組みがあるかどうか判定する.
 *
 *
 * BFS ではTLE になった.
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

    for a in 0..pos.len() {
        for b in a + 1..pos.len() {
            let (s, t) = (pos[a], pos[b]);
            // println!("{:?}, {:?}", &s, &t);
            // let mut ds = WeightedDisjointSet::new(h * w); // 0 indexed
            let mut q = VecDeque::new();
            q.push_back((s, 1));
            let mut used = vec![vec![false; w]; h];
            used[p.0 as usize][p.1 as usize] = true;
            while let Some((u, d)) = q.pop_back() {
                if t == u {
                    if d >= 3 {
                        println!("Yes");
                        return;
                    } else {
                        continue;
                    }
                }
                let (i, j) = u;
                used[i as usize][j as usize] = true;

                for (di, dj) in &neis {
                    let (ni, nj) = (i as isize + di, j as isize + dj);
                    if (ni < 0 || ni >= h as isize)
                        || (nj < 0 || nj >= w as isize)
                        || g[ni as usize][nj as usize] == '#'
                        || used[ni as usize][nj as usize]
                    {
                        continue;
                    }
                    q.push_back(((ni, nj), d + 1))
                }
            }
        }
    }

    println!("No");
}
