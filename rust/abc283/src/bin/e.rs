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

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut g: [[usize; w];h]
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            let mut no = true;
            let moves = vec![(1, 0), (-(1isize), 0), (0, 1), (0, -(1isize))];
            for (di, dj) in moves {
                let (ni, nj) = (i as isize + di, j as isize + dj);
                if (0 <= nj && nj < w as isize) && (0 <= ni && ni < h as isize) {
                    no &= g[i][j] != g[ni as usize][nj as usize];
                }
            }
            if no {
                // 孤立した要素、発見
                ans += 1;
                // ans += i;
                for j in 0..w {
                    let m = g[i][j];
                    g[i][j] = 1 - m;
                }
                break;
            }
        }
        // break ここまで
    }

    // 再チェック
    for i in 0..h {
        for j in 0..w {
            let mut no = true;
            let moves = vec![(1, 0), (-(1isize), 0), (0, 1), (0, -(1isize))];
            for (di, dj) in moves {
                let (ni, nj) = (i as isize + di, j as isize + dj);
                if (0 <= nj && nj < w as isize) && (0 <= ni && ni < h as isize) {
                    no &= g[i][j] != g[ni as usize][nj as usize];
                }
            }
            if no {
                // 孤立した要素、発見
                println!("-1");
                return;
            }
        }
        // break ここまで
    }

    println!("{}", ans);
}
