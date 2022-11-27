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
use library::structure::disjoint_set::*;

/**
 * E - Round Trip
 *
 * https://atcoder.jp/contests/abc276/tasks/abc276_e
 *
 * tags: #UnionFind
 *
 * 左上から順に #とS を除くマスをつなげていって、
 * S と隣接するマスのいずれか２つを連結できていれば Yes
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

    // let mut st = 0;
    let neis = [(0, -1isize), (-1isize, 0), (0, 1), (1, 0)];
    let mut pos = vec![];
    for i in 0..h {
        for j in 0..w {
            if g[i][j] == 'S' {
                // st = calc(i as isize, j as isize);
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
    let mut ds = WeightedDisjointSet::new(h * w); // 0 indexed

    for i in 0..h {
        for j in 0..w {
            if g[i][j] == '#' || g[i][j] == 'S' {
                continue;
            }
            for (di, dj) in &neis {
                let (ni, nj) = (i as isize + di, j as isize + dj);
                if ni < 0
                    || ni >= h as isize
                    || nj < 0
                    || nj >= w as isize
                    || g[ni as usize][nj as usize] == '#'
                    || g[ni as usize][nj as usize] == 'S'
                {
                    continue;
                }
                let pos = calc(i as isize, j as isize);
                let nei_pos = calc(ni, nj);
                ds.merge(pos, nei_pos, 1);
            }
        }
    }
    // S を含まないことを確認
    // for p in pos {
    //     println!("{}", ds.same(st, calc(p.0 as isize, p.1 as isize)));
    // }

    for i in 0..pos.len() {
        for j in i + 1..pos.len() {
            let (a, b) = (pos[i], pos[j]);
            let (p1, p2) = (calc(a.0, a.1), calc(b.0, b.1));
            if ds.same(p1, p2) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
