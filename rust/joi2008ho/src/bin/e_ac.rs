use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use superslice::{self, Ext};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * E - ペンキの色
 *
 * https://atcoder.jp/contests/joi2008ho/tasks/joi2008ho_e
 *
 * tags: #座標圧縮 #compress #累積和 #imos #全探索
 *
 * 右と下に余白を作る. それ以外は通常に２元座標圧縮で良い.
 * 圧縮した後の座標をもとに累積和をして、テープが貼られる区間を1 で塗って、余白はそのままにすると良い.
 * 今回は下にも余白を作るとメモリを使ってしまうため、右だけに余白を入れる
 */

// #[fastout]
fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize, // テープの数
        v: [(usize, usize, usize, usize); n] // 座標は全て生
    }

    // 座圧前のテープをプロットした座標
    // let mut tt = vec![vec![0; w + 1]; h + 1];
    // for &(x1, y1, x2, y2) in &v {
    //     tt[y1][x1] += 1; // 上左
    //     tt[y1][x2] -= 1; // 上右
    //     tt[y2][x2] += 1; // 下右
    //     tt[y2][x1] -= 1; // 下左
    // }
    // // x軸方向に累積和 (y 軸方向にひとつ多めに流す.)
    // for i in 0..=h {
    //     for j in 0..w {
    //         tt[i][j + 1] += tt[i][j];
    //     }
    // }
    // // y軸方向に累積和 (x 軸方向にひとつ多めに流す.)
    // for j in 0..=w {
    //     for i in 0..h {
    //         tt[i + 1][j] += tt[i][j];
    //     }
    // }
    // println!();
    // for xs in &tt {
    //     println!("{:?}", xs);
    // }

    let mut xs = vec![0, w];
    let mut ys = vec![0, h];
    // 左下の角の座標, 右上の角の座標
    for &(x1, y1, x2, y2) in &v {
        xs.append(&mut vec![x1, x2]);
        ys.append(&mut vec![y1, y2]);
        // 座標が範囲内であれば、右と上に余白を入れる
        if x2 + 1 < w {
            xs.push(x2 + 1);
        }
        // 上に余白を入れると MLE
        // if y2 + 1 < h {
        //     xs.push(y2 + 1);
        // }
    }

    xs = xs.into_iter().unique().sorted().collect_vec();
    ys = ys.into_iter().unique().sorted().collect_vec();

    // 座圧後の各左下、右上の角の頂点、更にその角がテープで覆われているか否かがもとまる
    let mut compressed = vec![(0, 0, 0, 0); n];
    for i in 0..n {
        let (x1, y1, x2, y2) = v[i];
        let px1 = xs.lower_bound(&x1) as isize;
        let py1 = ys.lower_bound(&y1) as isize;
        let px2 = xs.lower_bound(&x2) as isize;
        let py2 = ys.lower_bound(&y2) as isize;

        compressed[i] = (px1, py1, px2, py2);
    }

    // 座標の端の角（右上）はマスとしてカウントされないから、-1 してあらかじめ探索範囲から除く
    let x_len = xs.len() - 1;
    let y_len = ys.len() - 1;
    let mut g = vec![vec![0; x_len + 1]; y_len + 1];
    for &(x1, y1, x2, y2) in &compressed {
        g[y1 as usize][x1 as usize] += 1; // 上左
        g[y1 as usize][x2 as usize] -= 1; // 上右
        g[y2 as usize][x2 as usize] += 1; // 下右
        g[y2 as usize][x1 as usize] -= 1; // 下左
    }

    // imos 法
    for i in 0..=y_len {
        for j in 0..x_len {
            g[i][j + 1] += g[i][j];
        }
    }
    for j in 0..=x_len {
        for i in 0..y_len {
            g[i + 1][j] += g[i][j];
        }
    }

    // println!();
    // println!("y: {}", y_len);
    // for xs in &g {
    //     println!("{:?}", xs);
    // }

    // テープが貼られていない地点（imos 法の累積和の結果 0 になっているマス）のグループを探索する
    let mut q = VecDeque::new();
    let mut marker = 0;
    let mut used = vec![vec![-1; x_len]; y_len];
    for i in 0..y_len {
        for j in 0..x_len {
            // まだ通ってないなら, 新しいグループの始点とする
            if used[i][j] == -1 && g[i][j] == 0 {
                marker += 1;
                q.push_back(((i, j), marker));
            } else {
                continue;
            }
            while let Some(((y, x), marker)) = q.pop_back() {
                let moves = vec![(1, 0), (-(1isize), 0), (0, 1), (0, -(1isize))];
                for (di, dj) in moves {
                    let (ni, nj) = (y as isize + di, x as isize + dj);
                    if (0 <= nj && nj < x_len as isize)
                        && (0 <= ni && ni < y_len as isize)
                        && g[ni as usize][nj as usize] == 0
                        && used[ni as usize][nj as usize] == -1
                    {
                        q.push_back(((ni as usize, nj as usize), marker));
                        used[ni as usize][nj as usize] = marker;
                    }
                }
            }
        }
    }

    println!("{}", marker);
}
