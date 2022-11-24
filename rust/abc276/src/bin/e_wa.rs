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

use abc276::utils::*;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        t: [Chars; h]
    }

    let mut ds = WeightedDisjointSet::new(h * w); // 0 indexed

    // 2d の盤を1d で表現するために計算. h=5, w=3, 0index, (i,j)=(1,2) 1*3 + 2 =5
    let calc = |i: usize, j: usize| i * w + j;
    let mut s = 0;
    for i in 0..h {
        for j in 0..w {
            if t[i][j] == 'S' {
                s = calc(i, j);
                break;
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if t[i][j] == '#' || t[i][j] == 'S' {
                continue;
            }

            let nei = [(0, -1isize), (-1isize, 0), (0, 1), (1, 0)];

            let mut conn = 0;
            let pos = calc(i, j);

            for (di, dj) in &nei {
                let (ni, nj) = (i as isize + di, j as isize + dj);
                if ni < 0
                    || ni >= h as isize
                    || nj < 0
                    || nj >= w as isize
                    || t[ni as usize][nj as usize] == '#'
                {
                    continue;
                }
                let nei_pos = calc(ni as usize, nj as usize);
                if ds.same(s, nei_pos) {
                    // 隣接するマスがS と結合してる
                    conn += 1;
                }
            }

            if conn >= 2 && !ds.same(s, pos) {
                let p = ds.find(s);

                if ds.size[p] >= 3 {
                    println!("Yes");
                    return;
                }
            }

            for (di, dj) in &nei {
                let (ni, nj) = (i as isize + di, j as isize + dj);
                if ni < 0
                    || ni >= h as isize
                    || nj < 0
                    || nj >= w as isize
                    || t[ni as usize][nj as usize] == '#'
                {
                    continue;
                }
                let nei_pos = calc(ni as usize, nj as usize);
                ds.merge(pos, nei_pos, 1);
            }
            // println!();
        }
    }
    println!("No");
}
