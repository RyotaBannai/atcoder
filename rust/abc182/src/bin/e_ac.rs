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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Akari
 *
 * https://atcoder.jp/contests/abc182/tasks/abc182_e
 *
 * tags: #累積和 #両方向から累積和
 *
 *
 * - 上下、左右を一度に累積和を取らない
 * -> 左右の累積和をとった後に、上下の累積和を取ると光がないx座標から上下に光が向いてしまって条件を満たさなくなってしまう
 * - 壁を -1500 以下にする
 * -> 累積和をとった時に一方にたくさんの光があった時に累積和が最大 1500 くらいになって、生の値を取るようになってしまう
 * - マスが負の時はbreak ではなくcontinue
 * -> 壁の先に光がある時に、その光の累積和は取りたいからbreak で終わらない. 壁の時は continue をして加算をskip する
 */

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        ab: [(usize, usize); n], // (i,j)
        cd: [(usize, usize); m], // (i,j)
    }

    let mut s = Set::new();
    let mut grid = vec![vec![0; w]; h];
    for &(c, d) in cd.iter() {
        grid[c - 1][d - 1] = std::isize::MIN;
    }
    for &(a, b) in ab.iter() {
        grid[a - 1][b - 1] += 1;
    }

    // 左->右
    for i in 0..h {
        for j in 0..w - 1 {
            if grid[i][j] <= 0 {
                continue;
            }
            grid[i][j + 1] += grid[i][j];
        }
    }
    // 右->左
    for i in 0..h {
        for j in (1..w).rev() {
            if grid[i][j] <= 0 {
                continue;
            }
            grid[i][j - 1] += grid[i][j];
        }
    }

    // 計算
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] > 0 {
                s.insert((i, j));
            }
        }
    }

    // 上下用に初期化
    grid = vec![vec![0; w]; h];
    for (c, d) in cd {
        grid[c - 1][d - 1] = std::isize::MIN;
    }
    for (a, b) in ab {
        grid[a - 1][b - 1] += 1;
    }
    // 上->下
    for j in 0..w {
        for i in 0..h - 1 {
            if grid[i][j] <= 0 {
                continue;
            }
            grid[i + 1][j] += grid[i][j];
        }
    }
    // 下->上
    for j in 0..w {
        for i in (1..h).rev() {
            if grid[i][j] <= 0 {
                continue;
            }
            grid[i - 1][j] += grid[i][j];
        }
    }

    // 計算
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] > 0 {
                s.insert((i, j));
            }
        }
    }

    println!("{}", s.len());
}
